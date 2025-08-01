use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use log::{info, error, warn};
use imap::Session;
use native_tls::TlsStream;
use std::net::TcpStream;
use crate::models::*;
use crate::database::Database;
use crate::email_service::EmailService;
use crate::attachment_service::AttachmentService;

pub struct InboxService {
    database: Arc<Database>,
    email_service: Arc<Mutex<EmailService>>,
    attachment_service: Arc<AttachmentService>,
}

impl InboxService {
    pub fn new(
        database: Arc<Database>,
        email_service: Arc<Mutex<EmailService>>,
        attachment_service: Arc<AttachmentService>,
    ) -> Self {
        Self {
            database,
            email_service,
            attachment_service,
        }
    }
    
    // Inbox Monitor Management
    pub fn create_inbox_monitor(&self, user_id: i32, monitor_data: CreateInboxMonitor) -> Result<InboxMonitor, AppError> {
        let conn = self.database.get_connection();
        
        // Verify the email account belongs to the user
        let account_exists = conn.query_row(
            "SELECT 1 FROM email_accounts WHERE id = ?1 AND user_id = ?2",
            [monitor_data.email_account_id, user_id],
            |_| Ok(())
        );
        
        if account_exists.is_err() {
            return Err(AppError::NotFound("Email account not found".to_string()));
        }
        
        let check_interval = monitor_data.check_interval.unwrap_or(300); // Default 5 minutes
        
        let mut stmt = conn.prepare(
            "INSERT INTO inbox_monitors (user_id, email_account_id, check_interval, auto_reply_template_id)
             VALUES (?1, ?2, ?3, ?4)"
        )?;
        
        let monitor_id = stmt.insert((
            user_id,
            monitor_data.email_account_id,
            check_interval,
            monitor_data.auto_reply_template_id,
        ))?;
        
        self.get_inbox_monitor(user_id, monitor_id as i32)
    }
    
    pub fn get_inbox_monitor(&self, user_id: i32, monitor_id: i32) -> Result<InboxMonitor, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, email_account_id, is_active, check_interval, last_check, auto_reply_template_id, created_at
             FROM inbox_monitors WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let monitor = stmt.query_row([monitor_id, user_id], |row| {
            Ok(InboxMonitor {
                id: row.get(0)?,
                user_id: row.get(1)?,
                email_account_id: row.get(2)?,
                is_active: row.get(3)?,
                check_interval: row.get(4)?,
                last_check: row.get(5)?,
                auto_reply_template_id: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        Ok(monitor)
    }
    
    pub fn get_user_inbox_monitors(&self, user_id: i32) -> Result<Vec<InboxMonitor>, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, email_account_id, is_active, check_interval, last_check, auto_reply_template_id, created_at
             FROM inbox_monitors WHERE user_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let monitor_iter = stmt.query_map([user_id], |row| {
            Ok(InboxMonitor {
                id: row.get(0)?,
                user_id: row.get(1)?,
                email_account_id: row.get(2)?,
                is_active: row.get(3)?,
                check_interval: row.get(4)?,
                last_check: row.get(5)?,
                auto_reply_template_id: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let mut monitors = Vec::new();
        for monitor in monitor_iter {
            monitors.push(monitor?);
        }
        
        Ok(monitors)
    }
    
    pub fn update_inbox_monitor(&self, user_id: i32, monitor_id: i32, monitor_data: CreateInboxMonitor) -> Result<InboxMonitor, AppError> {
        let conn = self.database.get_connection();
        
        let check_interval = monitor_data.check_interval.unwrap_or(300);
        
        conn.execute(
            "UPDATE inbox_monitors SET email_account_id = ?1, check_interval = ?2, auto_reply_template_id = ?3
             WHERE id = ?4 AND user_id = ?5",
            (
                monitor_data.email_account_id,
                check_interval,
                monitor_data.auto_reply_template_id,
                monitor_id,
                user_id,
            ),
        )?;
        
        self.get_inbox_monitor(user_id, monitor_id)
    }
    
    pub fn toggle_inbox_monitor(&self, user_id: i32, monitor_id: i32, is_active: bool) -> Result<InboxMonitor, AppError> {
        let conn = self.database.get_connection();
        
        conn.execute(
            "UPDATE inbox_monitors SET is_active = ?1 WHERE id = ?2 AND user_id = ?3",
            [is_active as i32, monitor_id, user_id],
        )?;
        
        self.get_inbox_monitor(user_id, monitor_id)
    }
    
    pub fn delete_inbox_monitor(&self, user_id: i32, monitor_id: i32) -> Result<(), AppError> {
        let conn = self.database.get_connection();
        
        let rows_affected = conn.execute(
            "DELETE FROM inbox_monitors WHERE id = ?1 AND user_id = ?2",
            [monitor_id, user_id],
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound("Inbox monitor not found".to_string()));
        }
        
        info!("Deleted inbox monitor {} for user {}", monitor_id, user_id);
        Ok(())
    }
    
    // Email checking functionality
    pub async fn check_inbox(&self, user_id: i32, account_id: i32) -> Result<Vec<InboxEmail>, AppError> {
        // Get email account details
        let account_data = {
            let conn = self.database.get_connection();
            let mut stmt = conn.prepare(
                "SELECT email_address, imap_server, imap_port, username, password_encrypted
                 FROM email_accounts WHERE id = ?1 AND user_id = ?2"
            )?;
            
            stmt.query_row([account_id, user_id], |row| {
                Ok((
                    row.get::<_, String>(0)?, // email_address
                    row.get::<_, Option<String>>(1)?, // imap_server
                    row.get::<_, Option<i32>>(2)?, // imap_port
                    row.get::<_, String>(3)?, // username
                    row.get::<_, String>(4)?, // password_encrypted
                ))
            })?
        };
        
        let (email_address, imap_server, imap_port, username, password_encrypted) = account_data;
        
        let imap_server = imap_server.ok_or_else(|| AppError::Config("IMAP server not configured".to_string()))?;
        let imap_port = imap_port.unwrap_or(993);
        
        // Decrypt password (you'll need to implement this based on your encryption service)
        // For now, assuming password is stored in plain text (not recommended for production)
        let password = password_encrypted; // TODO: Decrypt this
        
        // Connect to IMAP server
        let emails = self.fetch_emails_from_imap(&imap_server, imap_port, &username, &password).await?;
        
        // Update last check time
        {
            let conn = self.database.get_connection();
            conn.execute(
                "UPDATE inbox_monitors SET last_check = CURRENT_TIMESTAMP WHERE email_account_id = ?1 AND user_id = ?2",
                [account_id, user_id],
            )?;
        }
        
        Ok(emails)
    }
    
    async fn fetch_emails_from_imap(
        &self,
        server: &str,
        port: i32,
        username: &str,
        password: &str,
    ) -> Result<Vec<InboxEmail>, AppError> {
        // This is a simplified IMAP implementation
        // In a real application, you'd want more robust error handling and connection management
        
        let tls = native_tls::TlsConnector::builder().build()
            .map_err(|e| AppError::Email(format!("TLS error: {}", e)))?;
        
        let client = imap::connect((server, port as u16), server, &tls)
            .map_err(|e| AppError::Email(format!("IMAP connection error: {}", e)))?;
        
        let mut session = client.login(username, password)
            .map_err(|e| AppError::Email(format!("IMAP login error: {:?}", e.0)))?;
        
        // Select INBOX
        session.select("INBOX")
            .map_err(|e| AppError::Email(format!("Failed to select INBOX: {}", e)))?;
        
        // Search for recent unread emails
        let sequences = session.search("UNSEEN")
            .map_err(|e| AppError::Email(format!("IMAP search error: {}", e)))?;
        
        let mut emails = Vec::new();
        
        // Limit to last 50 emails to avoid overwhelming the system
        let mut sequences: Vec<_> = sequences.into_iter().collect();
        sequences.reverse();
        let sequences: Vec<_> = sequences.into_iter().take(50).collect();
        
        if !sequences.is_empty() {
            let messages = session.fetch(sequences.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(","), "(ENVELOPE BODY[])") 
                .map_err(|e| AppError::Email(format!("IMAP fetch error: {}", e)))?;
            
            for message in messages.iter() {
                if let Some(envelope) = message.envelope() {
                    let subject = envelope.subject
                        .and_then(|s| std::str::from_utf8(s).ok())
                        .unwrap_or("No Subject")
                        .to_string();
                    
                    let sender = envelope.from
                        .as_ref()
                        .and_then(|addrs| addrs.first())
                        .and_then(|addr| {
                            let name = addr.name.and_then(|n| std::str::from_utf8(n).ok());
                            let mailbox = addr.mailbox.and_then(|m| std::str::from_utf8(m).ok());
                            let host = addr.host.and_then(|h| std::str::from_utf8(h).ok());
                            
                            match (name, mailbox, host) {
                                (Some(name), Some(mailbox), Some(host)) => Some(format!("{} <{}@{}>", name, mailbox, host)),
                                (None, Some(mailbox), Some(host)) => Some(format!("{}@{}", mailbox, host)),
                                _ => None,
                            }
                        })
                        .unwrap_or_else(|| "Unknown Sender".to_string());
                    
                    let received_at = envelope.date
                        .and_then(|d| std::str::from_utf8(d).ok())
                        .and_then(|d| chrono::DateTime::parse_from_rfc2822(d).ok())
                        .map(|d| d.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|| Utc::now());
                    
                    let body = message.body()
                        .and_then(|b| std::str::from_utf8(b).ok())
                        .unwrap_or("")
                        .to_string();
                    
                    let email = InboxEmail {
                        id: message.message.to_string(),
                        subject,
                        sender,
                        received_at,
                        body,
                        attachments: Vec::new(), // TODO: Parse attachments
                        is_read: false,
                    };
                    
                    emails.push(email);
                }
            }
        }
        
        session.logout()
            .map_err(|e| AppError::Email(format!("IMAP logout error: {}", e)))?;
        
        Ok(emails)
    }
    
    pub async fn process_automation_rules(&self, user_id: i32, email: &InboxEmail) -> Result<(), AppError> {
        // Get active automation rules for the user
        let conn = self.database.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, rule_name, keywords, conditions, actions
             FROM automation_rules WHERE user_id = ?1 AND is_active = 1"
        )?;
        
        let rule_iter = stmt.query_map([user_id], |row| {
            Ok((
                row.get::<_, i32>(0)?, // id
                row.get::<_, String>(1)?, // rule_name
                row.get::<_, String>(2)?, // keywords
                row.get::<_, String>(3)?, // conditions
                row.get::<_, String>(4)?, // actions
            ))
        })?;
        
        for rule_result in rule_iter {
            let (rule_id, rule_name, keywords_str, conditions_str, actions_str) = rule_result?;
            
            // Parse keywords
            let keywords: Vec<String> = serde_json::from_str(&keywords_str)
                .unwrap_or_else(|_| vec![]);
            
            // Check if email matches any keywords
            let matches_keywords = keywords.iter().any(|keyword| {
                email.subject.to_lowercase().contains(&keyword.to_lowercase()) ||
                email.body.to_lowercase().contains(&keyword.to_lowercase())
            });
            
            if matches_keywords {
                info!("Email matches automation rule '{}' for user {}", rule_name, user_id);
                
                // Parse and execute actions
                if let Ok(actions) = serde_json::from_str::<serde_json::Value>(&actions_str) {
                    self.execute_automation_actions(user_id, email, &actions).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn execute_automation_actions(
        &self,
        user_id: i32,
        email: &InboxEmail,
        actions: &serde_json::Value,
    ) -> Result<(), AppError> {
        if let Some(actions_array) = actions.as_array() {
            for action in actions_array {
                if let Some(action_type) = action.get("type").and_then(|t| t.as_str()) {
                    match action_type {
                        "auto_reply" => {
                            if let Some(template_id) = action.get("template_id").and_then(|t| t.as_i64()) {
                                self.send_auto_reply(user_id, email, template_id as i32).await?;
                            }
                        },
                        "mark_as_read" => {
                            // TODO: Mark email as read in IMAP
                            info!("Would mark email {} as read", email.id);
                        },
                        "move_to_folder" => {
                            if let Some(folder) = action.get("folder").and_then(|f| f.as_str()) {
                                // TODO: Move email to specified folder
                                info!("Would move email {} to folder {}", email.id, folder);
                            }
                        },
                        _ => {
                            warn!("Unknown automation action type: {}", action_type);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn send_auto_reply(
        &self,
        user_id: i32,
        original_email: &InboxEmail,
        template_id: i32,
    ) -> Result<(), AppError> {
        // Get template
        let conn = self.database.get_connection();
        let mut stmt = conn.prepare(
            "SELECT subject, body FROM email_templates WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let template_data = stmt.query_row([template_id, user_id], |row| {
            Ok((
                row.get::<_, Option<String>>(0)?, // subject
                row.get::<_, Option<String>>(1)?, // body
            ))
        })?;
        
        let (template_subject, template_body) = template_data;
        
        // Extract sender email from original email
        let sender_email = self.extract_email_address(&original_email.sender)?;
        
        // Create reply email
        let reply_subject = template_subject
            .unwrap_or_else(|| format!("Re: {}", original_email.subject));
        
        let reply_body = template_body
            .unwrap_or_else(|| "Thank you for your email. This is an automated response.".to_string());
        
        let email_message = EmailMessage {
            to: vec![sender_email],
            cc: None,
            bcc: None,
            subject: reply_subject,
            body: reply_body,
            attachments: None,
        };
        
        // Send the reply (you'll need to implement this based on your email service)
        // For now, just log it
        info!("Would send auto-reply to {} for email {}", original_email.sender, original_email.id);
        
        Ok(())
    }
    
    fn extract_email_address(&self, sender_string: &str) -> Result<String, AppError> {
        // Extract email address from "Name <email@domain.com>" format
        if let Some(start) = sender_string.find('<') {
            if let Some(end) = sender_string.find('>') {
                if end > start {
                    return Ok(sender_string[start + 1..end].to_string());
                }
            }
        }
        
        // If no angle brackets, assume the whole string is an email
        if sender_string.contains('@') {
            Ok(sender_string.trim().to_string())
        } else {
            Err(AppError::Validation("Could not extract email address from sender".to_string()))
        }
    }
}