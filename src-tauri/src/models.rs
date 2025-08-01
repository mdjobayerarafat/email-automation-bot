use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAccount {
    pub id: i32,
    pub user_id: i32,
    pub account_name: String,
    pub email_address: String,
    pub imap_server: Option<String>,
    pub imap_port: Option<i32>,
    pub smtp_server: Option<String>,
    pub smtp_port: Option<i32>,
    pub username: String,
    pub password_encrypted: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailAccount {
    pub account_name: String,
    pub email_address: String,
    pub imap_server: Option<String>,
    pub imap_port: Option<i32>,
    pub smtp_server: Option<String>,
    pub smtp_port: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailAccountWithUser {
    pub user_id: i32,
    pub account_name: String,
    pub email_address: String,
    pub imap_server: Option<String>,
    pub imap_port: Option<i32>,
    pub smtp_server: Option<String>,
    pub smtp_port: Option<i32>,
    pub username: String,
    pub password_encrypted: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailTemplate {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub template_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailTemplate {
    pub name: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub template_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailTemplateWithUser {
    pub user_id: i32,
    pub name: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub template_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutomationRule {
    pub id: i32,
    pub user_id: i32,
    pub rule_name: String,
    pub keywords: Vec<String>,
    pub conditions: serde_json::Value,
    pub actions: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
struct AutomationRuleRow {
    pub id: i32,
    pub user_id: i32,
    pub rule_name: String,
    pub keywords: String,
    pub conditions: String,
    pub actions: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAutomationRule {
    pub rule_name: String,
    pub keywords: Vec<String>,
    pub conditions: serde_json::Value,
    pub actions: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAutomationRuleWithUser {
    pub user_id: i32,
    pub rule_name: String,
    pub keywords: Vec<String>,
    pub conditions: serde_json::Value,
    pub actions: serde_json::Value,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailLog {
    pub id: i32,
    pub user_id: i32,
    pub email_account_id: Option<i32>,
    pub direction: String, // 'sent' or 'received'
    pub recipient_email: Option<String>,
    pub sender_email: Option<String>,
    pub subject: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailLog {
    pub user_id: i32,
    pub email_account_id: Option<i32>,
    pub direction: String,
    pub recipient_email: Option<String>,
    pub sender_email: Option<String>,
    pub subject: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledEmail {
    pub id: i32,
    pub user_id: i32,
    pub template_id: Option<i32>,
    pub recipient_list: Vec<String>,
    pub scheduled_time: DateTime<Utc>,
    pub recurrence_pattern: Option<String>,
    pub status: String, // 'pending', 'sent', 'failed'
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
struct ScheduledEmailRow {
    pub id: i32,
    pub user_id: i32,
    pub template_id: Option<i32>,
    pub recipient_list: String,
    pub scheduled_time: DateTime<Utc>,
    pub recurrence_pattern: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScheduledEmail {
    pub template_id: Option<i32>,
    pub recipient_list: Vec<String>,
    pub scheduled_time: DateTime<Utc>,
    pub recurrence_pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScheduledEmailWithUser {
    pub user_id: i32,
    pub template_id: Option<i32>,
    pub recipient_list: Vec<String>,
    pub scheduled_time: DateTime<Utc>,
    pub recurrence_pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMessage {
    pub to: Vec<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub subject: String,
    pub body: String,
    pub attachments: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchEmailRequest {
    pub template_id: i32,
    pub recipients: Vec<RecipientData>,
    pub schedule_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientData {
    pub email: String,
    pub variables: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailStats {
    pub total_sent: i32,
    pub total_received: i32,
    pub total_failed: i32,
    pub automation_rules_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionTest {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Email error: {0}")]
    Email(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Config error: {0}")]
    Config(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

// New models for enhanced features

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAttachment {
    pub id: i32,
    pub user_id: i32,
    pub email_log_id: i32,
    pub filename: String,
    pub original_filename: String,
    pub file_path: String,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub sender_email: Option<String>,
    pub received_at: Option<DateTime<Utc>>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailAttachment {
    pub user_id: i32,
    pub email_log_id: i32,
    pub filename: String,
    pub original_filename: String,
    pub file_path: String,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub sender_email: Option<String>,
    pub received_at: Option<DateTime<Utc>>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactList {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContactList {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    pub id: i32,
    pub user_id: i32,
    pub contact_list_id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub custom_fields: Option<serde_json::Value>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContact {
    pub contact_list_id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub custom_fields: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportContactsRequest {
    pub contact_list_id: i32,
    pub csv_data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailCampaign {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub template_id: Option<i32>,
    pub contact_list_id: Option<i32>,
    pub status: String,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub total_recipients: i32,
    pub sent_count: i32,
    pub failed_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailCampaign {
    pub name: String,
    pub template_id: Option<i32>,
    pub contact_list_id: Option<i32>,
    pub scheduled_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboxMonitor {
    pub id: i32,
    pub user_id: i32,
    pub email_account_id: i32,
    pub is_active: bool,
    pub check_interval: i32,
    pub last_check: Option<DateTime<Utc>>,
    pub auto_reply_template_id: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInboxMonitor {
    pub email_account_id: i32,
    pub check_interval: Option<i32>,
    pub auto_reply_template_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InboxEmail {
    pub id: String,
    pub subject: String,
    pub sender: String,
    pub received_at: DateTime<Utc>,
    pub body: String,
    pub attachments: Vec<String>,
    pub is_read: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportLogsRequest {
    pub format: String, // 'csv' or 'json'
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub status_filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentCategory {
    pub category: String,
    pub count: i32,
    pub total_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sent: i32,
    pub total_received: i32,
    pub total_failed: i32,
    pub automation_rules_count: i32,
    pub active_campaigns: i32,
    pub total_contacts: i32,
    pub attachment_categories: Vec<AttachmentCategory>,
    pub recent_activity: Vec<EmailLog>,
}