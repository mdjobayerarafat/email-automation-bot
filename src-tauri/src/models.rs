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