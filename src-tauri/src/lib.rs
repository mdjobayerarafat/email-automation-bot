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
mod attachment_service;
mod contact_service;
mod inbox_service;
mod campaign_service;

use models::*;
use database::Database;
use auth::AuthService;
use email_service::EmailService;
use encryption::EncryptionService;
use scheduler::SchedulerService;
use attachment_service::AttachmentService;
use contact_service::ContactService;
use inbox_service::InboxService;
use campaign_service::CampaignService;

// Application state
#[derive(Clone)]
struct AppState {
    database: Arc<Database>,
    auth_service: Arc<AuthService>,
    email_service: Arc<Mutex<EmailService>>,
    encryption_service: Arc<EncryptionService>,
    scheduler_service: Arc<SchedulerService>,
    attachment_service: Arc<AttachmentService>,
    contact_service: Arc<ContactService>,
    inbox_service: Arc<InboxService>,
    campaign_service: Arc<CampaignService>,
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
     
     let attachment_service = Arc::new(
         AttachmentService::new(Arc::clone(&database))
     );
     
     let contact_service = Arc::new(
         ContactService::new(Arc::clone(&database))
     );
     
     let inbox_service = Arc::new(
         InboxService::new(
             Arc::clone(&database),
             Arc::clone(&email_service),
             Arc::clone(&attachment_service),
         )
     );
     
     let campaign_service = Arc::new(
         CampaignService::new(
             Arc::clone(&database),
             Arc::clone(&email_service),
             Arc::clone(&contact_service),
         )
     );
     
     // Store database in app state
     let app_state = AppState {
         database,
         auth_service,
         email_service,
         encryption_service,
         scheduler_service,
         attachment_service,
         contact_service,
         inbox_service,
         campaign_service,
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

// Contact Management Commands
#[tauri::command]
fn create_contact_list(
    state: tauri::State<'_, AppState>,
    token: String,
    list_data: CreateContactList,
) -> Result<ContactList, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.contact_service.create_contact_list(user.id, list_data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_contact_lists(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<ContactList>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.contact_service.get_user_contact_lists(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn import_contacts(
    state: tauri::State<'_, AppState>,
    token: String,
    import_data: ImportContactsRequest,
) -> Result<String, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    let imported_count = state.contact_service.import_contacts_from_csv(user.id, import_data)?
        .len();
    Ok(format!("Successfully imported {} contacts", imported_count))
}

#[tauri::command]
fn get_contacts(
    state: tauri::State<'_, AppState>,
    token: String,
    list_id: i32,
) -> Result<Vec<Contact>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.contact_service.get_contacts_by_list(user.id, list_id)
        .map_err(|e| e.to_string())
}

// Inbox Monitor Commands
#[tauri::command]
fn create_inbox_monitor(
    state: tauri::State<'_, AppState>,
    token: String,
    monitor_data: CreateInboxMonitor,
) -> Result<InboxMonitor, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.inbox_service.create_inbox_monitor(user.id, monitor_data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_inbox_monitors(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<InboxMonitor>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.inbox_service.get_user_inbox_monitors(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_inbox(
    state: tauri::State<'_, AppState>,
    token: String,
    account_id: i32,
) -> Result<Vec<InboxEmail>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.inbox_service.check_inbox(user.id, account_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_inbox_monitor(
    state: tauri::State<'_, AppState>,
    token: String,
    monitor_id: i32,
    is_active: bool,
) -> Result<InboxMonitor, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.inbox_service.toggle_inbox_monitor(user.id, monitor_id, is_active)
        .map_err(|e| e.to_string())
}

// Campaign Management Commands
#[tauri::command]
fn create_campaign(
    state: tauri::State<'_, AppState>,
    token: String,
    campaign_data: CreateEmailCampaign,
) -> Result<EmailCampaign, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.campaign_service.create_campaign(user.id, campaign_data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_campaigns(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<EmailCampaign>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.campaign_service.get_user_campaigns(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn send_campaign(
    state: tauri::State<'_, AppState>,
    token: String,
    batch_request: BatchEmailRequest,
) -> Result<String, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.campaign_service.send_batch_emails(user.id, batch_request).await
        .map_err(|e| e.to_string())?;
    Ok("Campaign sent successfully".to_string())
}

#[tauri::command]
fn get_campaign_stats(
    state: tauri::State<'_, AppState>,
    token: String,
    campaign_id: i32,
) -> Result<campaign_service::CampaignStats, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.campaign_service.get_campaign_stats(user.id, campaign_id)
        .map_err(|e| e.to_string())
}

// Attachment Management Commands
#[tauri::command]
fn get_attachments(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<EmailAttachment>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.attachment_service.get_user_attachments(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_attachment_categories(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<Vec<AttachmentCategory>, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.attachment_service.get_attachment_categories(user.id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_attachment(
    state: tauri::State<'_, AppState>,
    token: String,
    attachment_id: i32,
) -> Result<String, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    state.attachment_service.delete_attachment(user.id, attachment_id)
        .map_err(|e| e.to_string())?;
    Ok("Attachment deleted successfully".to_string())
}

// Export Commands
#[tauri::command]
fn export_logs(
    state: tauri::State<'_, AppState>,
    token: String,
    export_request: ExportLogsRequest,
) -> Result<String, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    // Get logs based on filters
    let logs = state.database.get_email_logs(user.id, export_request.limit)
        .map_err(|e| e.to_string())?;
    
    match export_request.format.as_str() {
        "csv" => {
            // TODO: Implement CSV export
            Ok("CSV export not yet implemented".to_string())
        },
        "json" => {
            let json_data = serde_json::to_string_pretty(&logs)
                .map_err(|e| format!("Failed to serialize logs: {}", e))?;
            Ok(json_data)
        },
        _ => Err("Unsupported export format".to_string())
    }
}

// Dashboard Stats
#[tauri::command]
fn get_dashboard_stats(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Result<DashboardStats, String> {
    let user = state.auth_service.extract_user_from_token(&token)?;
    
    // Get various stats for dashboard
    let email_stats = state.database.get_email_stats(user.id)
        .map_err(|e| e.to_string())?;
    
    let contact_count = state.contact_service.get_total_contacts(user.id)
        .unwrap_or(0);
    
    let template_count = state.database.get_email_templates(user.id)
        .map(|templates| templates.len() as i32)
        .unwrap_or(0);
    
    let campaign_count = state.campaign_service.get_user_campaigns(user.id)
        .map(|campaigns| campaigns.len() as i32)
        .unwrap_or(0);
    
    let monitor_count = state.inbox_service.get_user_inbox_monitors(user.id)
        .map(|monitors| monitors.len() as i32)
        .unwrap_or(0);
    
    Ok(DashboardStats {
        total_emails_sent: email_stats.total_sent,
        total_emails_failed: email_stats.total_failed,
        total_contacts: contact_count,
        total_templates: template_count,
        total_campaigns: campaign_count,
        active_monitors: monitor_count,
        recent_activity: Vec::new(), // TODO: Implement recent activity
    })
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
            get_email_logs,
            // Contact Management
            create_contact_list,
            get_contact_lists,
            import_contacts,
            get_contacts,
            // Inbox Monitor
            create_inbox_monitor,
            get_inbox_monitors,
            check_inbox,
            toggle_inbox_monitor,
            // Campaign Management
            create_campaign,
            get_campaigns,
            send_campaign,
            get_campaign_stats,
            // Attachment Management
            get_attachments,
            get_attachment_categories,
            delete_attachment,
            // Export
            export_logs,
            // Dashboard
            get_dashboard_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
