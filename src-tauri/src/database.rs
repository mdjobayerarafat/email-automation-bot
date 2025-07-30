use rusqlite::{Connection, Result as SqliteResult, params};
use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn: Arc::new(Mutex::new(conn)) };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Create users table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            [],
        )?;

        // Create email_accounts table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS email_accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
                account_name TEXT NOT NULL,
                email_address TEXT NOT NULL,
                imap_server TEXT,
                imap_port INTEGER,
                smtp_server TEXT,
                smtp_port INTEGER,
                username TEXT NOT NULL,
                password_encrypted TEXT NOT NULL,
                is_active BOOLEAN DEFAULT 1,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            [],
        )?;

        // Create email_templates table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS email_templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                subject TEXT,
                body TEXT,
                template_type TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            [],
        )?;

        // Create automation_rules table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS automation_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
                rule_name TEXT NOT NULL,
                keywords TEXT,
                conditions TEXT,
                actions TEXT,
                is_active BOOLEAN DEFAULT 1,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            [],
        )?;

        // Create email_logs table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS email_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
                email_account_id INTEGER REFERENCES email_accounts(id) ON DELETE SET NULL,
                direction TEXT CHECK (direction IN ('sent', 'received')),
                recipient_email TEXT,
                sender_email TEXT,
                subject TEXT,
                status TEXT,
                error_message TEXT,
                sent_at TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            [],
        )?;

        // Create scheduled_emails table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS scheduled_emails (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
                template_id INTEGER REFERENCES email_templates(id) ON DELETE SET NULL,
                recipient_list TEXT,
                scheduled_time TEXT,
                recurrence_pattern TEXT,
                status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed')),
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            [],
        )?;

        Ok(())
    }

    // User operations
    pub fn create_user(&self, user: CreateUser) -> Result<User> {
        let password_hash = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST)?;
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            r#"
            INSERT INTO users (username, email, password_hash, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![&user.username, &user.email, &password_hash, &now, &now],
        )?;

        let user_id = conn.last_insert_rowid() as i32;
        
        Ok(User {
            id: user_id,
            username: user.username,
            email: user.email,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE email = ?1"
        )?;
        
        let user_iter = stmt.query_map([email], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        for user in user_iter {
            return Ok(Some(user?));
        }
        
        Ok(None)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = ?1"
        )?;
        
        let user_iter = stmt.query_map([user_id], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        for user in user_iter {
            return Ok(Some(user?));
        }
        
        Ok(None)
    }

    // Email account operations
    pub fn create_email_account(&self, account: CreateEmailAccountWithUser) -> Result<EmailAccount> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            r#"
            INSERT INTO email_accounts (user_id, account_name, email_address, imap_server, imap_port, smtp_server, smtp_port, username, password_encrypted, is_active, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                account.user_id,
                &account.account_name,
                &account.email_address,
                &account.imap_server,
                account.imap_port,
                &account.smtp_server,
                account.smtp_port,
                &account.username,
                &account.password_encrypted,
                account.is_active.unwrap_or(true),
                &now
            ],
        )?;

        let account_id = conn.last_insert_rowid() as i32;
        
        Ok(EmailAccount {
            id: account_id,
            user_id: account.user_id,
            account_name: account.account_name,
            email_address: account.email_address,
            imap_server: account.imap_server,
            imap_port: account.imap_port,
            smtp_server: account.smtp_server,
            smtp_port: account.smtp_port,
            username: account.username,
            password_encrypted: account.password_encrypted,
            is_active: account.is_active.unwrap_or(true),
            created_at: Utc::now(),
        })
    }

    pub fn get_email_accounts(&self, user_id: i32) -> Result<Vec<EmailAccount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, account_name, email_address, imap_server, imap_port, smtp_server, smtp_port, username, password_encrypted, is_active, created_at FROM email_accounts WHERE user_id = ?1"
        )?;
        
        let account_iter = stmt.query_map([user_id], |row| {
            Ok(EmailAccount {
                id: row.get(0)?,
                user_id: row.get(1)?,
                account_name: row.get(2)?,
                email_address: row.get(3)?,
                imap_server: row.get(4)?,
                imap_port: row.get(5)?,
                smtp_server: row.get(6)?,
                smtp_port: row.get(7)?,
                username: row.get(8)?,
                password_encrypted: row.get(9)?,
                is_active: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?;

        let mut accounts = Vec::new();
        for account in account_iter {
            accounts.push(account?);
        }
        
        Ok(accounts)
    }

    pub fn get_email_account(&self, user_id: i32, account_id: i32) -> Result<Option<EmailAccount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, account_name, email_address, imap_server, imap_port, smtp_server, smtp_port, username, password_encrypted, is_active, created_at FROM email_accounts WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let account_iter = stmt.query_map([account_id, user_id], |row| {
            Ok(EmailAccount {
                id: row.get(0)?,
                user_id: row.get(1)?,
                account_name: row.get(2)?,
                email_address: row.get(3)?,
                imap_server: row.get(4)?,
                imap_port: row.get(5)?,
                smtp_server: row.get(6)?,
                smtp_port: row.get(7)?,
                username: row.get(8)?,
                password_encrypted: row.get(9)?,
                is_active: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?;

        for account in account_iter {
            return Ok(Some(account?));
        }
        
        Ok(None)
    }

    // Email template operations
    pub fn create_email_template(&self, template: CreateEmailTemplateWithUser) -> Result<EmailTemplate> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            r#"
            INSERT INTO email_templates (user_id, name, subject, body, template_type, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                template.user_id,
                &template.name,
                &template.subject,
                &template.body,
                &template.template_type,
                &now,
                &now
            ],
        )?;

        let template_id = conn.last_insert_rowid() as i32;
        
        Ok(EmailTemplate {
            id: template_id,
            user_id: template.user_id,
            name: template.name,
            subject: template.subject,
            body: template.body,
            template_type: template.template_type,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn get_email_templates(&self, user_id: i32) -> Result<Vec<EmailTemplate>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, subject, body, template_type, created_at, updated_at FROM email_templates WHERE user_id = ?1"
        )?;
        
        let template_iter = stmt.query_map([user_id], |row| {
            Ok(EmailTemplate {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                subject: row.get(3)?,
                body: row.get(4)?,
                template_type: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut templates = Vec::new();
        for template in template_iter {
            templates.push(template?);
        }
        
        Ok(templates)
    }

    pub fn get_email_template(&self, template_id: i32, user_id: i32) -> Result<Option<EmailTemplate>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, subject, body, template_type, created_at, updated_at FROM email_templates WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let template_iter = stmt.query_map([template_id, user_id], |row| {
            Ok(EmailTemplate {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                subject: row.get(3)?,
                body: row.get(4)?,
                template_type: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        for template in template_iter {
            return Ok(Some(template?));
        }
        
        Ok(None)
    }

    // Automation rule operations
    pub fn create_automation_rule(&self, rule: CreateAutomationRuleWithUser) -> Result<AutomationRule> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        
        let keywords_json = serde_json::to_string(&rule.keywords)
            .map_err(|e| anyhow::anyhow!("Failed to serialize keywords: {}", e))?;
        let conditions_json = serde_json::to_string(&rule.conditions)
            .map_err(|e| anyhow::anyhow!("Failed to serialize conditions: {}", e))?;
        let actions_json = serde_json::to_string(&rule.actions)
            .map_err(|e| anyhow::anyhow!("Failed to serialize actions: {}", e))?;
        
        conn.execute(
            r#"
            INSERT INTO automation_rules (user_id, rule_name, keywords, conditions, actions, is_active, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                rule.user_id,
                &rule.rule_name,
                &keywords_json,
                &conditions_json,
                &actions_json,
                rule.is_active.unwrap_or(true),
                &now
            ],
        )?;

        let rule_id = conn.last_insert_rowid() as i32;
        
        Ok(AutomationRule {
            id: rule_id,
            user_id: rule.user_id,
            rule_name: rule.rule_name,
            keywords: rule.keywords,
            conditions: rule.conditions,
            actions: rule.actions,
            is_active: rule.is_active.unwrap_or(true),
            created_at: Utc::now(),
        })
    }

    pub fn get_automation_rules(&self, user_id: i32) -> Result<Vec<AutomationRule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, rule_name, keywords, conditions, actions, is_active, created_at FROM automation_rules WHERE user_id = ?1"
        )?;
        
        let rule_iter = stmt.query_map([user_id], |row| {
            Ok(AutomationRule {
                id: row.get(0)?,
                user_id: row.get(1)?,
                rule_name: row.get(2)?,
                keywords: serde_json::from_str(&row.get::<_, String>(3)?)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(3, "keywords".to_string(), rusqlite::types::Type::Text))?,

                conditions: serde_json::from_str(&row.get::<_, String>(4)?)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(4, "conditions".to_string(), rusqlite::types::Type::Text))?,
                actions: serde_json::from_str(&row.get::<_, String>(5)?)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(5, "actions".to_string(), rusqlite::types::Type::Text))?,
                is_active: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let mut rules = Vec::new();
        for rule in rule_iter {
            rules.push(rule?);
        }
        Ok(rules)
    }

    // Email logging operations
    pub fn log_email(&self, log: CreateEmailLog) -> Result<EmailLog> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            r#"
            INSERT INTO email_logs (user_id, email_account_id, direction, recipient_email, sender_email, subject, status, error_message, sent_at, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                log.user_id,
                log.email_account_id,
                &log.direction,
                &log.recipient_email,
                &log.sender_email,
                &log.subject,
                &log.status,
                &log.error_message,
                &log.sent_at,
                &now
            ],
        )?;

        let log_id = conn.last_insert_rowid() as i32;
        
        Ok(EmailLog {
            id: log_id,
            user_id: log.user_id,
            email_account_id: log.email_account_id,
            direction: log.direction,
            recipient_email: log.recipient_email,
            sender_email: log.sender_email,
            subject: log.subject,
            status: log.status,
            error_message: log.error_message,
            sent_at: log.sent_at,
            created_at: Utc::now(),
        })
    }

    pub fn get_email_logs(&self, user_id: i32, limit: Option<i32>) -> Result<Vec<EmailLog>> {
        let conn = self.conn.lock().unwrap();
        let query = if let Some(limit) = limit {
            format!("SELECT id, user_id, email_account_id, direction, recipient_email, sender_email, subject, status, error_message, sent_at, created_at FROM email_logs WHERE user_id = ?1 ORDER BY created_at DESC LIMIT {}", limit)
        } else {
            "SELECT id, user_id, email_account_id, direction, recipient_email, sender_email, subject, status, error_message, sent_at, created_at FROM email_logs WHERE user_id = ?1 ORDER BY created_at DESC".to_string()
        };
        
        let mut stmt = conn.prepare(&query)?;
        let log_iter = stmt.query_map([user_id], |row| {
            Ok(EmailLog {
                id: row.get(0)?,
                user_id: row.get(1)?,
                email_account_id: row.get(2)?,
                direction: row.get(3)?,
                recipient_email: row.get(4)?,
                sender_email: row.get(5)?,
                subject: row.get(6)?,
                status: row.get(7)?,
                error_message: row.get(8)?,
                sent_at: row.get(9)?,
                created_at: row.get(10)?,
            })
        })?;
        
        let mut logs = Vec::new();
        for log in log_iter {
            logs.push(log?);
        }
        Ok(logs)
    }

    // Scheduled email operations
    pub fn get_pending_scheduled_emails(&self) -> Result<Vec<ScheduledEmail>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, template_id, recipient_list, scheduled_time, recurrence_pattern, status, created_at FROM scheduled_emails WHERE status = 'pending' AND scheduled_time <= datetime('now')"
        )?;
        
        let scheduled_email_iter = stmt.query_map([], |row| {
            let recipient_list_str: String = row.get(3)?;
            let recipient_list: Vec<String> = serde_json::from_str(&recipient_list_str)
                .map_err(|_| rusqlite::Error::InvalidColumnType(3, "recipient_list".to_string(), rusqlite::types::Type::Text))?;
            
            let scheduled_time_str: String = row.get(4)?;
            let scheduled_time = DateTime::parse_from_rfc3339(&scheduled_time_str)
                .map_err(|_| rusqlite::Error::InvalidColumnType(4, "scheduled_time".to_string(), rusqlite::types::Type::Text))?
                .with_timezone(&Utc);
            
            Ok(ScheduledEmail {
                id: row.get(0)?,
                user_id: row.get(1)?,
                template_id: row.get(2)?,
                recipient_list,
                scheduled_time,
                recurrence_pattern: row.get(5)?,
                status: row.get(6)?,
                created_at: Utc::now(),
            })
        })?;
        
        let mut emails = Vec::new();
        for email in scheduled_email_iter {
            emails.push(email?);
        }
        Ok(emails)
    }

    pub fn update_scheduled_email_status(&self, email_id: i32, status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE scheduled_emails SET status = ?1 WHERE id = ?2",
            params![status, email_id],
        )?;
        Ok(())
    }

    pub fn create_scheduled_email(&self, email: CreateScheduledEmailWithUser) -> Result<ScheduledEmail> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();
        
        let recipient_list_json = serde_json::to_string(&email.recipient_list)
            .map_err(|e| anyhow::anyhow!("Failed to serialize recipient list: {}", e))?;
        let scheduled_time_str = email.scheduled_time.to_rfc3339();
        
        conn.execute(
            r#"
            INSERT INTO scheduled_emails (user_id, template_id, recipient_list, scheduled_time, recurrence_pattern, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                email.user_id,
                email.template_id,
                &recipient_list_json,
                &scheduled_time_str,
                &email.recurrence_pattern,
                "pending",
                &now
            ],
        )?;
        
        let email_id = conn.last_insert_rowid() as i32;
        
        Ok(ScheduledEmail {
            id: email_id,
            user_id: email.user_id,
            template_id: email.template_id,
            recipient_list: email.recipient_list,
            scheduled_time: email.scheduled_time,
            recurrence_pattern: email.recurrence_pattern,
            status: "pending".to_string(),
            created_at: Utc::now(),
        })
    }

    // Statistics operations
    pub fn get_email_stats(&self, user_id: i32) -> Result<EmailStats> {
        let conn = self.conn.lock().unwrap();
        
        let total_sent: i32 = conn.query_row(
            "SELECT COUNT(*) FROM email_logs WHERE user_id = ?1 AND direction = 'sent'",
            [user_id],
            |row| row.get(0)
        ).unwrap_or(0);
        
        let total_received: i32 = conn.query_row(
            "SELECT COUNT(*) FROM email_logs WHERE user_id = ?1 AND direction = 'received'",
            [user_id],
            |row| row.get(0)
        ).unwrap_or(0);
        
        let total_failed: i32 = conn.query_row(
            "SELECT COUNT(*) FROM email_logs WHERE user_id = ?1 AND status = 'failed'",
            [user_id],
            |row| row.get(0)
        ).unwrap_or(0);
        
        let automation_rules_count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM automation_rules WHERE user_id = ?1 AND is_active = 1",
            [user_id],
            |row| row.get(0)
        ).unwrap_or(0);
        
        Ok(EmailStats {
            total_sent,
            total_received,
            total_failed,
            automation_rules_count,
        })
    }
}