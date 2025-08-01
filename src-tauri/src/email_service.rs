use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header::ContentType, Mailbox, MultiPart, SinglePart};
use imap::Session;
use std::net::TcpStream;
use native_tls::{TlsConnector, TlsStream};
use crate::models::*;
use anyhow::Result;
use std::collections::HashMap;
use regex::Regex;
use tera::{Tera, Context};
use chrono::{Utc, Datelike, Timelike};

pub struct EmailService {
    template_engine: Tera,
}

impl EmailService {
    pub fn new() -> Self {
        let mut tera = Tera::new("templates/**/*").unwrap_or_else(|_| Tera::default());
        
        // Add built-in templates
        tera.add_raw_template("welcome", include_str!("../templates/welcome.html")).ok();
        tera.add_raw_template("auto_reply", include_str!("../templates/auto_reply.html")).ok();
        
        EmailService {
            template_engine: tera,
        }
    }

    pub async fn test_smtp_connection(&self, account: &EmailAccount, password: &str) -> Result<ConnectionTest> {
        match self.create_smtp_transport(account, password) {
            Ok(mailer) => {
                match mailer.test_connection() {
                    Ok(true) => Ok(ConnectionTest {
                        success: true,
                        message: "SMTP connection successful".to_string(),
                    }),
                    Ok(false) => Ok(ConnectionTest {
                        success: false,
                        message: "SMTP connection failed".to_string(),
                    }),
                    Err(e) => Ok(ConnectionTest {
                        success: false,
                        message: format!("SMTP connection error: {}", e),
                    }),
                }
            }
            Err(e) => Ok(ConnectionTest {
                success: false,
                message: format!("Failed to create SMTP transport: {}", e),
            }),
        }
    }

    pub async fn test_imap_connection(&self, account: &EmailAccount, password: &str) -> Result<ConnectionTest> {
        let imap_server = account.imap_server.as_ref().ok_or_else(|| {
            anyhow::anyhow!("IMAP server not configured")
        })?;
        
        let imap_port = account.imap_port.unwrap_or(993);
        
        match self.create_imap_session(imap_server, imap_port as u16, &account.username, password) {
            Ok(_) => Ok(ConnectionTest {
                success: true,
                message: "IMAP connection successful".to_string(),
            }),
            Err(e) => Ok(ConnectionTest {
                success: false,
                message: format!("IMAP connection error: {}", e),
            }),
        }
    }

    pub async fn send_email(&self, account: &EmailAccount, password: &str, email: &EmailMessage) -> Result<()> {
        let mailer = self.create_smtp_transport(account, password)?;
        
        let from_mailbox: Mailbox = format!("{} <{}>", account.account_name, account.email_address)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid from address: {}", e))?;
        
        let mut message_builder = Message::builder()
            .from(from_mailbox)
            .subject(&email.subject);
        
        // Add recipients
        for to_addr in &email.to {
            let to_mailbox: Mailbox = to_addr.parse()
                .map_err(|e| anyhow::anyhow!("Invalid to address {}: {}", to_addr, e))?;
            message_builder = message_builder.to(to_mailbox);
        }
        
        // Add CC recipients
        if let Some(cc_list) = &email.cc {
            for cc_addr in cc_list {
                let cc_mailbox: Mailbox = cc_addr.parse()
                    .map_err(|e| anyhow::anyhow!("Invalid CC address {}: {}", cc_addr, e))?;
                message_builder = message_builder.cc(cc_mailbox);
            }
        }
        
        // Add BCC recipients
        if let Some(bcc_list) = &email.bcc {
            for bcc_addr in bcc_list {
                let bcc_mailbox: Mailbox = bcc_addr.parse()
                    .map_err(|e| anyhow::anyhow!("Invalid BCC address {}: {}", bcc_addr, e))?;
                message_builder = message_builder.bcc(bcc_mailbox);
            }
        }
        
        // Create message body
        let message = if email.body.contains("<html>") || email.body.contains("<HTML>") {
            // HTML email
            message_builder
                .multipart(
                    MultiPart::alternative()
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_PLAIN)
                                .body(self.html_to_text(&email.body))
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(ContentType::TEXT_HTML)
                                .body(email.body.clone())
                        )
                )?
        } else {
            // Plain text email
            message_builder
                .body(email.body.clone())?
        };
        
        mailer.send(&message)
            .map_err(|e| anyhow::anyhow!("Failed to send email: {}", e))?;
        
        Ok(())
    }

    pub async fn send_batch_emails(&mut self, account: &EmailAccount, password: &str, template: &EmailTemplate, recipients: &[RecipientData]) -> Result<Vec<String>> {
        let mut results = Vec::new();
        let mailer = self.create_smtp_transport(account, password)?;
        
        for recipient in recipients.iter() {
            match self.send_templated_email(&mailer, account, template, recipient).await {
                Ok(_) => results.push(format!("Success: {}", recipient.email)),
                Err(e) => results.push(format!("Failed {}: {}", recipient.email, e)),
            }
            
            // Add delay to prevent rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(results)
    }

    async fn send_templated_email(&mut self, mailer: &SmtpTransport, account: &EmailAccount, template: &EmailTemplate, recipient: &RecipientData) -> Result<()> {
        let mut context = Context::new();
        
        // Add recipient variables to context
        for (key, value) in &recipient.variables {
            context.insert(key, value);
        }
        
        // Add default variables
        context.insert("email", &recipient.email);
        context.insert("date", &Utc::now().format("%Y-%m-%d").to_string());
        
        // Render subject and body
        let subject = if let Some(subj) = &template.subject {
            self.template_engine.render_str(subj, &context)
                .unwrap_or_else(|_| subj.clone())
        } else {
            "No Subject".to_string()
        };
        
        let body = if let Some(body_template) = &template.body {
            self.template_engine.render_str(body_template, &context)
                .unwrap_or_else(|_| body_template.clone())
        } else {
            "No Content".to_string()
        };
        
        let from_mailbox: Mailbox = format!("{} <{}>", account.account_name, account.email_address)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid from address: {}", e))?;
        
        let to_mailbox: Mailbox = recipient.email.parse()
            .map_err(|e| anyhow::anyhow!("Invalid to address: {}", e))?;
        
        let message = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .body(body)?;
        
        mailer.send(&message)
            .map_err(|e| anyhow::anyhow!("Failed to send email: {}", e))?;
        
        Ok(())
    }

    pub async fn check_emails(&self, account: &EmailAccount, password: &str) -> Result<Vec<EmailMessage>> {
        let imap_server = account.imap_server.as_ref().ok_or_else(|| {
            anyhow::anyhow!("IMAP server not configured")
        })?;
        
        let imap_port = account.imap_port.unwrap_or(993);
        let mut session = self.create_imap_session(imap_server, imap_port as u16, &account.username, password)?;
        
        // Select INBOX
        session.select("INBOX")?;
        
        // Search for unseen emails
        let messages = session.search("UNSEEN")?;
        let mut emails = Vec::new();
        
        for msg_id in messages.iter().take(10) { // Limit to 10 recent emails
            if let Ok(messages) = session.fetch(msg_id.to_string(), "(ENVELOPE BODY[TEXT])") {
                for message in messages.iter() {
                    if let Some(envelope) = message.envelope() {
                        let subject = envelope.subject
                            .and_then(|s| std::str::from_utf8(s).ok())
                            .unwrap_or("No Subject")
                            .to_string();
                        
                        let _from = envelope.from
                            .as_ref()
                            .and_then(|addrs| addrs.first())
                            .and_then(|addr| addr.mailbox)
                            .and_then(|mb| std::str::from_utf8(mb).ok())
                            .unwrap_or("Unknown")
                            .to_string();
                        
                        let body = message.text()
                            .and_then(|b| std::str::from_utf8(b).ok())
                            .unwrap_or("No Content")
                            .to_string();
                        
                        emails.push(EmailMessage {
                            to: vec![account.email_address.clone()],
                            cc: None,
                            bcc: None,
                            subject,
                            body,
                            attachments: None,
                        });
                    }
                }
            }
        }
        
        session.logout()?;
        Ok(emails)
    }

    pub async fn process_automation_rules(&self, email: &EmailMessage, rules: &[AutomationRule]) -> Vec<serde_json::Value> {
        let mut triggered_actions = Vec::new();
        
        for rule in rules {
            if !rule.is_active {
                continue;
            }
            
            let mut rule_triggered = false;
            
            // Check keywords in subject and body
            let email_content = format!("{} {}", email.subject.to_lowercase(), email.body.to_lowercase());
            
            for keyword in &rule.keywords {
                if email_content.contains(&keyword.to_lowercase()) {
                    rule_triggered = true;
                    break;
                }
            }
            
            // Check additional conditions
            if rule_triggered {
                if let Ok(conditions) = serde_json::from_value::<HashMap<String, serde_json::Value>>(rule.conditions.clone()) {
                    // Check sender conditions
                    if let Some(sender_pattern) = conditions.get("sender_pattern") {
                        if let Some(pattern_str) = sender_pattern.as_str() {
                            if let Ok(regex) = Regex::new(pattern_str) {
                                if let Some(from_addr) = email.to.first() {
                                    if !regex.is_match(from_addr) {
                                        rule_triggered = false;
                                    }
                                }
                            }
                        }
                    }
                    
                    // Check time conditions (business hours, etc.)
                    if let Some(time_condition) = conditions.get("business_hours_only") {
                        if time_condition.as_bool().unwrap_or(false) {
                            let now = Utc::now();
                            let hour = now.hour();
                            let weekday = now.weekday().number_from_monday();
                            
                            if weekday > 5 || hour < 9 || hour > 17 {
                                rule_triggered = false;
                            }
                        }
                    }
                }
            }
            
            if rule_triggered {
                triggered_actions.push(rule.actions.clone());
            }
        }
        
        triggered_actions
    }

    fn create_smtp_transport(&self, account: &EmailAccount, password: &str) -> Result<SmtpTransport> {
        let smtp_server = account.smtp_server.as_ref().ok_or_else(|| {
            anyhow::anyhow!("SMTP server not configured")
        })?;
        
        let smtp_port = account.smtp_port.unwrap_or(587);
        
        let creds = Credentials::new(account.username.clone(), password.to_string());
        
        let mailer = SmtpTransport::relay(smtp_server)?
            .port(smtp_port as u16)
            .credentials(creds)
            .build();
        
        Ok(mailer)
    }

    fn create_imap_session(&self, server: &str, port: u16, username: &str, password: &str) -> Result<Session<TlsStream<TcpStream>>> {
        let tls = TlsConnector::builder().build()?;
        let client = imap::connect((server, port), server, &tls)?;
        let session = client.login(username, password)
            .map_err(|e| anyhow::anyhow!("IMAP login failed: {:?}", e.0))?;
        
        Ok(session)
    }

    fn html_to_text(&self, html: &str) -> String {
        // Simple HTML to text conversion
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(html, "").to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_text() {
        let service = EmailService::new();
        let html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
        let text = service.html_to_text(html);
        assert_eq!(text, "HelloWorld");
    }

    #[test]
    fn test_template_rendering() {
        let service = EmailService::new();
        let mut context = Context::new();
        context.insert("name", "John");
        
        let template = "Hello {{name}}!";
        let result = service.template_engine.render_str(template, &context).unwrap();
        assert_eq!(result, "Hello John!");
    }
}