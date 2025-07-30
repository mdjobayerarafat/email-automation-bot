use cron::Schedule;
use chrono::{DateTime, Utc, Duration};
use std::str::FromStr;
use tokio::time::{interval, Duration as TokioDuration};
use crate::models::*;
use crate::database::Database;
use crate::email_service::EmailService;
use crate::encryption::EncryptionService;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error, warn};

pub struct SchedulerService {
    database: Arc<Database>,
    email_service: Arc<Mutex<EmailService>>,
    encryption_service: Arc<EncryptionService>,
    is_running: Arc<Mutex<bool>>,
}

impl SchedulerService {
    pub fn new(
        database: Arc<Database>,
        email_service: Arc<Mutex<EmailService>>,
        encryption_service: Arc<EncryptionService>,
    ) -> Self {
        SchedulerService {
            database,
            email_service,
            encryption_service,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start(&self) -> Result<(), AppError> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        info!("Starting email scheduler service");
        
        let database = Arc::clone(&self.database);
        let email_service = Arc::clone(&self.email_service);
        let encryption_service = Arc::clone(&self.encryption_service);
        let is_running_flag = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(60)); // Check every minute
            
            loop {
                interval.tick().await;
                
                // Check if we should stop
                {
                    let running = is_running_flag.lock().await;
                    if !*running {
                        break;
                    }
                }
                
                if let Err(e) = Self::process_scheduled_emails(
                    &database,
                    &email_service,
                    &encryption_service,
                ).await {
                    error!("Error processing scheduled emails: {}", e);
                }
            }
            
            info!("Email scheduler service stopped");
        });

        Ok(())
    }

    pub async fn stop(&self) {
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
        info!("Stopping email scheduler service");
    }

    async fn process_scheduled_emails(
        database: &Database,
        email_service: &Arc<Mutex<EmailService>>,
        encryption_service: &EncryptionService,
    ) -> Result<(), AppError> {
        let pending_emails = database.get_pending_scheduled_emails()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        
        for scheduled_email in pending_emails {
            match Self::send_scheduled_email(
                database,
                email_service,
                encryption_service,
                &scheduled_email,
            ).await {
                Ok(_) => {
                    info!("Successfully sent scheduled email ID: {}", scheduled_email.id);
                    
                    // Handle recurrence
                    if let Some(pattern) = &scheduled_email.recurrence_pattern {
                        if let Ok(next_time) = Self::calculate_next_occurrence(pattern, &scheduled_email.scheduled_time) {
                            let next_scheduled = CreateScheduledEmailWithUser {
                                user_id: scheduled_email.user_id,
                                template_id: scheduled_email.template_id,
                                recipient_list: scheduled_email.recipient_list.clone(),
                                scheduled_time: next_time,
                                recurrence_pattern: Some(pattern.clone()),
                            };
                            
                            if let Err(e) = database.create_scheduled_email(next_scheduled) {
                                error!("Failed to create next occurrence for email ID {}: {}", scheduled_email.id, e);
                            }
                        }
                    }
                    
                    // Mark current email as sent
                    if let Err(e) = database.update_scheduled_email_status(scheduled_email.id, "sent") {
                        error!("Failed to update email status for ID {}: {}", scheduled_email.id, e);
                    }
                }
                Err(e) => {
                    error!("Failed to send scheduled email ID {}: {}", scheduled_email.id, e);
                    
                    // Mark as failed
                    if let Err(update_err) = database.update_scheduled_email_status(scheduled_email.id, "failed") {
                        error!("Failed to update failed email status for ID {}: {}", scheduled_email.id, update_err);
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn send_scheduled_email(
        database: &Database,
        email_service: &Arc<Mutex<EmailService>>,
        encryption_service: &EncryptionService,
        scheduled_email: &ScheduledEmail,
    ) -> Result<(), AppError> {
        // Get user's email accounts
        let email_accounts = database.get_email_accounts(scheduled_email.user_id)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let active_account = email_accounts.into_iter()
            .find(|acc| acc.is_active)
            .ok_or_else(|| AppError::NotFound("No active email account found".to_string()))?;
        
        // Decrypt password
        let password = encryption_service.decrypt(&active_account.password_encrypted)
            .map_err(|e| AppError::Internal(format!("Failed to decrypt password: {}", e)))?;
        
        // Get template if specified
        let template = if let Some(template_id) = scheduled_email.template_id {
            database.get_email_template(scheduled_email.user_id, template_id)
                .map_err(|e| AppError::Internal(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Email template not found".to_string()))?
        } else {
            return Err(AppError::Validation("No template specified for scheduled email".to_string()));
        };
        
        // Create recipient data
        let recipients: Vec<RecipientData> = scheduled_email.recipient_list.iter()
            .map(|email| RecipientData {
                email: email.clone(),
                variables: std::collections::HashMap::new(),
            })
            .collect();
        
        // Send batch emails
        let mut email_service_guard = email_service.lock().await;
        let results = email_service_guard.send_batch_emails(&active_account, &password, &template, &recipients).await?;
        drop(email_service_guard);
        
        // Log results
        for result in results {
            let (status, recipient) = if result.starts_with("Success:") {
                ("success", result.strip_prefix("Success: ").unwrap_or(""))
            } else {
                ("failed", result.strip_prefix("Failed ").unwrap_or("").split(':').next().unwrap_or(""))
            };
            
            let log_entry = CreateEmailLog {
                user_id: scheduled_email.user_id,
                email_account_id: Some(active_account.id),
                direction: "sent".to_string(),
                recipient_email: Some(recipient.to_string()),
                sender_email: Some(active_account.email_address.clone()),
                subject: template.subject.clone(),
                status: status.to_string(),
                error_message: if status == "failed" { Some(result.clone()) } else { None },
                sent_at: Some(Utc::now()),
            };
            
            if let Err(e) = database.log_email(log_entry) {
                warn!("Failed to create email log: {}", e);
            }
        }
        
        Ok(())
    }

    fn calculate_next_occurrence(pattern: &str, last_time: &DateTime<Utc>) -> Result<DateTime<Utc>, AppError> {
        // Handle simple patterns first
        match pattern {
            "daily" => Ok(*last_time + Duration::days(1)),
            "weekly" => Ok(*last_time + Duration::weeks(1)),
            "monthly" => Ok(*last_time + Duration::days(30)), // Approximate
            "yearly" => Ok(*last_time + Duration::days(365)), // Approximate
            _ => {
                // Try to parse as cron expression
                if let Ok(schedule) = Schedule::from_str(pattern) {
                    if let Some(next) = schedule.upcoming(Utc).next() {
                        Ok(next)
                    } else {
                        Err(AppError::Validation("No upcoming occurrence found for cron pattern".to_string()))
                    }
                } else {
                    Err(AppError::Validation(format!("Invalid recurrence pattern: {}", pattern)))
                }
            }
        }
    }

    pub fn validate_cron_pattern(pattern: &str) -> bool {
        Schedule::from_str(pattern).is_ok()
    }

    pub fn get_next_occurrences(pattern: &str, count: usize) -> Result<Vec<DateTime<Utc>>, AppError> {
        match pattern {
            "daily" | "weekly" | "monthly" | "yearly" => {
                let mut occurrences = Vec::new();
                let mut current = Utc::now();
                
                for _ in 0..count {
                    current = match pattern {
                        "daily" => current + Duration::days(1),
                        "weekly" => current + Duration::weeks(1),
                        "monthly" => current + Duration::days(30),
                        "yearly" => current + Duration::days(365),
                        _ => unreachable!(),
                    };
                    occurrences.push(current);
                }
                
                Ok(occurrences)
            }
            _ => {
                if let Ok(schedule) = Schedule::from_str(pattern) {
                    let occurrences: Vec<DateTime<Utc>> = schedule
                        .upcoming(Utc)
                        .take(count)
                        .collect();
                    Ok(occurrences)
                } else {
                    Err(AppError::Validation(format!("Invalid cron pattern: {}", pattern)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_validation() {
        assert!(SchedulerService::validate_cron_pattern("0 9 * * MON-FRI")); // Weekdays at 9 AM
        assert!(SchedulerService::validate_cron_pattern("0 0 1 * *")); // First day of month
        assert!(!SchedulerService::validate_cron_pattern("invalid pattern"));
    }

    #[test]
    fn test_simple_patterns() {
        let now = Utc::now();
        
        assert!(SchedulerService::calculate_next_occurrence("daily", &now).is_ok());
        assert!(SchedulerService::calculate_next_occurrence("weekly", &now).is_ok());
        assert!(SchedulerService::calculate_next_occurrence("monthly", &now).is_ok());
        assert!(SchedulerService::calculate_next_occurrence("yearly", &now).is_ok());
    }

    #[test]
    fn test_next_occurrences() {
        let occurrences = SchedulerService::get_next_occurrences("daily", 3).unwrap();
        assert_eq!(occurrences.len(), 3);
        
        // Check that each occurrence is one day after the previous
        for i in 1..occurrences.len() {
            let diff = occurrences[i] - occurrences[i-1];
            assert_eq!(diff.num_days(), 1);
        }
    }
}