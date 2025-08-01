-- Initial database schema for Email Automation Bot

-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Email accounts
CREATE TABLE email_accounts (
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Email templates
CREATE TABLE email_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    subject TEXT,
    body TEXT,
    template_type TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Automation rules
CREATE TABLE automation_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    rule_name TEXT NOT NULL,
    keywords TEXT,
    conditions TEXT,
    actions TEXT,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Email logs
CREATE TABLE email_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    email_account_id INTEGER REFERENCES email_accounts(id) ON DELETE SET NULL,
    direction TEXT CHECK (direction IN ('sent', 'received')),
    recipient_email TEXT,
    sender_email TEXT,
    subject TEXT,
    status TEXT,
    error_message TEXT,
    sent_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Scheduled emails
CREATE TABLE scheduled_emails (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    template_id INTEGER REFERENCES email_templates(id) ON DELETE SET NULL,
    recipient_list TEXT,
    scheduled_time DATETIME,
    recurrence_pattern TEXT,
    status TEXT DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Email attachments table for attachment parser feature
CREATE TABLE email_attachments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    email_log_id INTEGER REFERENCES email_logs(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    mime_type TEXT,
    sender_email TEXT,
    received_at DATETIME,
    category TEXT, -- 'document', 'image', 'archive', 'other'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Contact lists for batch email functionality
CREATE TABLE contact_lists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Contacts for batch email functionality
CREATE TABLE contacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    contact_list_id INTEGER REFERENCES contact_lists(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,
    custom_fields TEXT, -- JSON for additional fields
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Batch email campaigns
CREATE TABLE email_campaigns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    template_id INTEGER REFERENCES email_templates(id) ON DELETE SET NULL,
    contact_list_id INTEGER REFERENCES contact_lists(id) ON DELETE SET NULL,
    status TEXT DEFAULT 'draft' CHECK (status IN ('draft', 'scheduled', 'sending', 'completed', 'failed')),
    scheduled_time DATETIME,
    total_recipients INTEGER DEFAULT 0,
    sent_count INTEGER DEFAULT 0,
    failed_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Inbox monitoring settings
CREATE TABLE inbox_monitors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    email_account_id INTEGER REFERENCES email_accounts(id) ON DELETE CASCADE,
    is_active BOOLEAN DEFAULT 1,
    check_interval INTEGER DEFAULT 300, -- seconds
    last_check DATETIME,
    auto_reply_template_id INTEGER REFERENCES email_templates(id) ON DELETE SET NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_email_accounts_user_id ON email_accounts(user_id);
CREATE INDEX idx_email_accounts_active ON email_accounts(user_id, is_active);
CREATE INDEX idx_email_templates_user_id ON email_templates(user_id);
CREATE INDEX idx_automation_rules_user_id ON automation_rules(user_id);
CREATE INDEX idx_automation_rules_active ON automation_rules(user_id, is_active);
CREATE INDEX idx_email_logs_user_id ON email_logs(user_id);
CREATE INDEX idx_email_logs_created_at ON email_logs(created_at DESC);
CREATE INDEX idx_scheduled_emails_user_id ON scheduled_emails(user_id);
CREATE INDEX idx_scheduled_emails_status ON scheduled_emails(status);
CREATE INDEX idx_scheduled_emails_time ON scheduled_emails(scheduled_time);
CREATE INDEX idx_email_attachments_user_id ON email_attachments(user_id);
CREATE INDEX idx_email_attachments_email_log ON email_attachments(email_log_id);
CREATE INDEX idx_contact_lists_user_id ON contact_lists(user_id);
CREATE INDEX idx_contacts_user_id ON contacts(user_id);
CREATE INDEX idx_contacts_list_id ON contacts(contact_list_id);
CREATE INDEX idx_contacts_email ON contacts(email);
CREATE INDEX idx_email_campaigns_user_id ON email_campaigns(user_id);
CREATE INDEX idx_email_campaigns_status ON email_campaigns(status);
CREATE INDEX idx_inbox_monitors_user_id ON inbox_monitors(user_id);
CREATE INDEX idx_inbox_monitors_account ON inbox_monitors(email_account_id);

-- SQLite doesn't support functions and triggers in the same way as PostgreSQL
-- Updated_at fields will need to be handled in the application code

-- Default email templates will be inserted by the application when needed
-- Removed INSERT statements to avoid foreign key constraint issues