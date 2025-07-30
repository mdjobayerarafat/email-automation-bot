use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;
use std::env;
use std::path::PathBuf;
use std::fs;
use tauri::Manager;
use dotenv::dotenv;
use chrono::Utc;

mod models;
mod database;
mod auth;
mod email_service;
mod encryption;
mod scheduler;

use models::*;
use database::Database;
use auth::AuthService;
use email_service::EmailService;
use encryption::EncryptionService;
use scheduler::SchedulerService;

// Application state
#[derive(Clone)]
struct AppState {
    database: Arc<Database>,
    auth_service: Arc<AuthService>,
    email_service: Arc<Mutex<EmailService>>,
    encryption_service: Arc<EncryptionService>,
    scheduler_service: Arc<SchedulerService>,
}

fn initialize_app(app_handle: tauri::AppHandle) -> Result<String, String> {
    info!("Initializing application...");
    
    // Get the app data directory using Tauri's path resolver
    let app_data_dir = match env::var("APPDATA") {
        Ok(appdata) => {
            let mut path = PathBuf::from(appdata);
            path.push("EmailAutomationBot");
            path
        }
        Err(_) => {
            // Fallback to home directory on non-Windows systems
            match env::var("HOME") {
                Ok(home) => {
                    let mut path = PathBuf::from(home);
                    path.push(".email_automation_bot");
                    path
                }
                Err(_) => {
                    return Err("Failed to determine app data directory".to_string());
                }
            }
        }
    };
    
    // Create the directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&app_data_dir) {
        return Err(format!("Failed to create app data directory: {}", e));
    }
    
    // Create the database path
    let db_path = app_data_dir.join("email_automation.db");
    
    info!("Database path: {}", db_path.display());
    
    // Initialize database
    let db = Database::new(&db_path)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    // Initialize services
     let database = Arc::new(db);
     let auth_service = Arc::new(
         AuthService::new()
             .map_err(|e| format!("Failed to initialize auth service: {}", e))?
     );
     let email_service = Arc::new(Mutex::new(EmailService::new()));
     let encryption_service = Arc::new(
         EncryptionService::new()
             .map_err(|e| format!("Failed to initialize encryption service: {}", e))?
     );
     let scheduler_service = Arc::new(
         SchedulerService::new(
             Arc::clone(&database),
             Arc::clone(&email_service),
             Arc::clone(&encryption_service),
         )
     );
     
     // Store database in app state
     let app_state = AppState {
         database,
         auth_service,
         email_service,
         encryption_service,
         scheduler_service,
     };
    
    app_handle.manage(app_state);
    
    Ok("Application initialized successfully".to_string())
}

// Authentication commands
#[tauri::command]
fn register_user(
    state: tauri::State<'_, AppState>,
    user_data: CreateUser,
) -> Result<UserInfo, String> {
    let user = state.database.create_user(user_data)
        .map_err(|e| e.to_string())?;
    
    Ok(UserInfo {
        id: user.id,
        username: user.username,
        email: user.email,
    })
}

#[tauri::command]
fn login_user(
    state: tauri::State<'_, AppState>,
    login_data: LoginRequest,
) -> Result<LoginResponse, String> {
    let user = state.database.get_user_by_email(&login_data.email)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
    
    if !state.auth_service.verify_password(&login_data.password, &user.password_hash)? {
        return Err("Invalid password".to_string());
    }
    
    let token = state.auth_service.generate_token(&user)?;
    
    Ok(LoginResponse {
        token,
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
        },
    })
}

#[tauri::command]
fn verify_token(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<UserInfo, String> {
    state.auth_service.extract_user_from_token(&token)
        .map_err(|e| e.to_string())
}

// Email account commands
#[tauri::command]
fn create_email_account(
    state: tauri::State<'_, AppState>,
    token: String,
    account_data: CreateEmailAccount,
) -> Result<EmailAccount, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    let encrypted_password = state.encryption_service.encrypt(&account_data.password)?;
    
    let account_with_user = CreateEmailAccountWithUser {
        user_id: user.id,
        account_name: account_data.account_name,
        email_address: account_data.email_address,
        imap_server: account_data.imap_server,
        imap_port: account_data.imap_port,
        smtp_server: account_data.smtp_server,
        smtp_port: account_data.smtp_port,
        username: account_data.username,
        password_encrypted: encrypted_password,
        is_active: Some(true),
    };
    
    state.database.create_email_account(account_with_user)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_email_accounts(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<EmailAccount>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.database.get_email_accounts(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn test_email_connection(
    _state: tauri::State<'_, AppState>,
    _token: String,
    _account_id: i32,
) -> Result<ConnectionTest, String> {
    // TODO: Implement email connection testing
    Ok(ConnectionTest {
        success: true,
        message: "Connection test not implemented yet".to_string(),
    })
}

// Email template commands
#[tauri::command]
fn create_email_template(
    state: tauri::State<'_, AppState>,
    token: String,
    template_data: CreateEmailTemplate,
) -> Result<EmailTemplate, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    
    let template_with_user = CreateEmailTemplateWithUser {
        user_id: user.id,
        name: template_data.name,
        subject: template_data.subject,
        body: template_data.body,
        template_type: template_data.template_type,
    };
    
    state.database.create_email_template(template_with_user)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_email_templates(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<EmailTemplate>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.database.get_email_templates(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_email_template(
    state: tauri::State<'_, AppState>,
    token: String,
    template_id: i32,
) -> Result<Option<EmailTemplate>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.database.get_email_template(template_id, user.id)
        .map_err(|e| e.to_string())
}

// Automation rule commands
#[tauri::command]
fn create_automation_rule(
    state: tauri::State<'_, AppState>,
    token: String,
    rule_data: CreateAutomationRule,
) -> Result<AutomationRule, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    
    // Create a new rule with user_id from token
    let rule_with_user = CreateAutomationRuleWithUser {
        user_id: user.id,
        rule_name: rule_data.rule_name,
        keywords: rule_data.keywords,
        conditions: rule_data.conditions,
        actions: rule_data.actions,
        is_active: Some(true),
    };
    
    state.database.create_automation_rule(rule_with_user)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_automation_rules(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<AutomationRule>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.database.get_automation_rules(user.id)
        .map_err(|e| e.to_string())
}

// Email operations
#[tauri::command]
async fn send_email(
    state: tauri::State<'_, AppState>,
    token: String,
    account_id: i32,
    email_data: EmailMessage,
) -> Result<String, String> {
    let user = state.auth_service.extract_user_from_token(&token)
        .map_err(|e| e.to_string())?;
    
    // Get the email account
    let account = state.database.get_email_account(user.id, account_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Email account not found".to_string())?;
    
    // Get the decrypted password
    let password = state.encryption_service.decrypt(&account.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Send the email using the email service
    let mut email_service = state.email_service.lock().await;
    email_service.send_email(&account, &password, &email_data).await
        .map_err(|e| format!("Failed to send email: {}", e))?;
    
    // Log the email
    let log_entry = CreateEmailLog {
        user_id: user.id,
        email_account_id: Some(account.id),
        direction: "sent".to_string(),
        recipient_email: Some(email_data.to.join(", ")),
        sender_email: Some(account.email_address.clone()),
        subject: Some(email_data.subject.clone()),
        status: "success".to_string(),
        error_message: None,
        sent_at: Some(chrono::Utc::now()),
    };
    
    if let Err(e) = state.database.log_email(log_entry) {
        eprintln!("Failed to log email: {}", e);
    }
    
    Ok("Email sent successfully".to_string())
}

#[tauri::command]
fn send_batch_emails(
    _state: tauri::State<'_, AppState>,
    _token: String,
    _batch_request: BatchEmailRequest,
) -> Result<Vec<String>, String> {
    // TODO: Implement batch email sending
    Ok(vec!["Batch email sending not implemented yet".to_string()])
}

#[tauri::command]
fn check_emails(
    _state: tauri::State<'_, AppState>,
    _token: String,
    _account_id: i32,
) -> Result<Vec<EmailMessage>, String> {
    // TODO: Implement email checking
    Ok(Vec::new())
}

// Scheduling commands
#[tauri::command]
fn create_scheduled_email(
    state: tauri::State<'_, AppState>,
    token: String,
    scheduled_email: CreateScheduledEmail,
) -> Result<ScheduledEmail, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    
    // Create a new scheduled email with user_id from token
    let scheduled_with_user = CreateScheduledEmailWithUser {
        user_id: user.id,
        template_id: scheduled_email.template_id,
        recipient_list: scheduled_email.recipient_list,
        scheduled_time: scheduled_email.scheduled_time,
        recurrence_pattern: scheduled_email.recurrence_pattern,
    };
    
    state.database.create_scheduled_email(scheduled_with_user)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn validate_cron_pattern(_pattern: String) -> Result<bool, String> {
    // TODO: Implement cron pattern validation
    Ok(true)
}

#[tauri::command]
fn get_next_occurrences(
    _pattern: String,
    _count: usize,
) -> Result<Vec<String>, String> {
    // TODO: Implement cron pattern next occurrences
    Ok(vec!["Next occurrence calculation not implemented yet".to_string()])
}

// Statistics and logs
#[tauri::command]
fn get_email_stats(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<EmailStats, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.database.get_email_stats(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_email_logs(
    state: tauri::State<'_, AppState>,
    token: String,
    limit: Option<i32>,
) -> Result<Vec<EmailLog>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.database.get_email_logs(user.id, limit)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logging
    env_logger::init();
    
    info!("Email Automation Bot starting...");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            if let Err(e) = initialize_app(app_handle.clone()) {
                eprintln!("Failed to initialize app: {}", e);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register_user,
            login_user,
            verify_token,
            create_email_account,
            get_email_accounts,
            test_email_connection,
            create_email_template,
            get_email_templates,
            get_email_template,
            create_automation_rule,
            get_automation_rules,
            send_email,
            send_batch_emails,
            check_emails,
            create_scheduled_email,
            validate_cron_pattern,
            get_next_occurrences,
            get_email_stats,
            get_email_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
