use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use log::{info, error, warn};
use crate::models::*;
use crate::database::Database;
use crate::email_service::EmailService;
use crate::contact_service::ContactService;
use std::collections::HashMap;
use tera::{Tera, Context};

pub struct CampaignService {
    database: Arc<Database>,
    email_service: Arc<Mutex<EmailService>>,
    contact_service: Arc<ContactService>,
}

impl CampaignService {
    pub fn new(
        database: Arc<Database>,
        email_service: Arc<Mutex<EmailService>>,
        contact_service: Arc<ContactService>,
    ) -> Self {
        Self {
            database,
            email_service,
            contact_service,
        }
    }
    
    // Email Campaign Management
    pub fn create_campaign(&self, user_id: i32, campaign_data: CreateEmailCampaign) -> Result<EmailCampaign, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "INSERT INTO email_campaigns (user_id, name, contact_list_id, template_id, status, scheduled_time)
             VALUES (?1, ?2, ?3, ?4, 'draft', ?5)"
        )?;
        
        let campaign_id = stmt.insert((
            user_id,
            &campaign_data.name,
            campaign_data.contact_list_id,
            campaign_data.template_id,
            campaign_data.scheduled_time,
        ))?;
        
        self.get_campaign(user_id, campaign_id as i32)
    }
    
    pub fn get_campaign(&self, user_id: i32, campaign_id: i32) -> Result<EmailCampaign, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, subject, body, contact_list_id, template_id, status, 
                    sent_count, total_count, scheduled_at, created_at, updated_at
             FROM email_campaigns WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let campaign = stmt.query_row([campaign_id, user_id], |row| {
            Ok(EmailCampaign {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                template_id: row.get(6)?,
                contact_list_id: row.get(5)?,
                status: row.get(7)?,
                scheduled_time: row.get(10)?,
                total_recipients: row.get(9)?,
                sent_count: row.get(8)?,
                failed_count: 0,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        
        Ok(campaign)
    }
    
    pub fn get_user_campaigns(&self, user_id: i32) -> Result<Vec<EmailCampaign>, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, contact_list_id, template_id, status, 
                    sent_count, total_recipients, failed_count, scheduled_time, created_at, updated_at
             FROM email_campaigns WHERE user_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let campaign_iter = stmt.query_map([user_id], |row| {
            Ok(EmailCampaign {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                contact_list_id: row.get(3)?,
                template_id: row.get(4)?,
                status: row.get(5)?,
                sent_count: row.get(6)?,
                total_recipients: row.get(7)?,
                failed_count: row.get(8)?,
                scheduled_time: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;
        
        let mut campaigns = Vec::new();
        for campaign in campaign_iter {
            campaigns.push(campaign?);
        }
        
        Ok(campaigns)
    }
    
    pub fn update_campaign(&self, user_id: i32, campaign_id: i32, campaign_data: CreateEmailCampaign) -> Result<EmailCampaign, AppError> {
        let conn = self.database.get_connection();
        
        conn.execute(
            "UPDATE email_campaigns SET name = ?1, contact_list_id = ?2, 
                    template_id = ?3, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?4 AND user_id = ?5 AND status = 'draft'",
            (
                &campaign_data.name,
                campaign_data.contact_list_id,
                campaign_data.template_id,
                campaign_id,
                user_id,
            ),
        )?;
        
        self.get_campaign(user_id, campaign_id)
    }
    
    pub fn delete_campaign(&self, user_id: i32, campaign_id: i32) -> Result<(), AppError> {
        let conn = self.database.get_connection();
        
        let rows_affected = conn.execute(
            "DELETE FROM email_campaigns WHERE id = ?1 AND user_id = ?2 AND status = 'draft'",
            [campaign_id, user_id],
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound("Campaign not found or cannot be deleted".to_string()));
        }
        
        info!("Deleted campaign {} for user {}", campaign_id, user_id);
        Ok(())
    }
    
    // Batch Email Sending
    pub async fn send_batch_emails(&self, user_id: i32, request: BatchEmailRequest) -> Result<(), AppError> {
        // Extract needed values before moving
        let template_id = request.template_id;
        let schedule_time = request.schedule_time;
        let recipients = request.recipients;
        
        if recipients.is_empty() {
            return Err(AppError::Validation("No recipients provided".to_string()));
        }
        
        // Create campaign record (scope the connection)
        let campaign_id = {
            let conn = self.database.get_connection();
            let mut stmt = conn.prepare(
                "INSERT INTO email_campaigns (user_id, name, status, total_recipients)
                 VALUES (?1, ?2, 'sending', ?3)"
            )?;
            
            let campaign_id = stmt.insert((
                user_id,
                format!("Batch Email - {}", Utc::now().format("%Y-%m-%d %H:%M")),
                recipients.len() as i32,
            ))? as i32;
            
            // Update campaign status and total count
            conn.execute(
                "UPDATE email_campaigns SET status = 'sending', total_recipients = ?1, updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2 AND user_id = ?3",
                [recipients.len() as i32, campaign_id, user_id],
            )?;
            
            campaign_id
        };
        
        let mut sent_count = 0;
        let mut failed_count = 0;
        
        // Send emails to each recipient
        for recipient in recipients {
            match self.send_personalized_email(user_id, template_id, schedule_time, &recipient, campaign_id).await {
                Ok(_) => {
                    sent_count += 1;
                    info!("Email sent successfully to {}", recipient.email);
                },
                Err(e) => {
                    failed_count += 1;
                    error!("Failed to send email to {}: {}", recipient.email, e);
                    
                    // Log the failure
                    self.log_email_failure(user_id, &recipient.email, "Email Template", &e.to_string())?;
                }
            }
            
            // Update campaign progress (scope the connection)
            {
                let conn = self.database.get_connection();
                conn.execute(
                    "UPDATE email_campaigns SET sent_count = ?1, updated_at = CURRENT_TIMESTAMP
                     WHERE id = ?2",
                    [sent_count, campaign_id],
                )?;
            }
            
            // Add a small delay to avoid overwhelming the SMTP server
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        // Update final campaign status (scope the connection)
        {
            let conn = self.database.get_connection();
            let final_status = if failed_count == 0 { "completed" } else { "partial" };
            conn.execute(
                "UPDATE email_campaigns SET status = ?1, updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                (final_status, campaign_id),
            )?;
        }
        
        info!(
            "Batch email campaign {} completed: {} sent, {} failed",
            campaign_id, sent_count, failed_count
        );
        
        Ok(())
    }
    
    async fn send_personalized_email(
        &self,
        user_id: i32,
        template_id: i32,
        schedule_time: Option<chrono::DateTime<chrono::Utc>>,
        recipient: &RecipientData,
        campaign_id: i32,
    ) -> Result<(), AppError> {
        // Get template data (scope the connection)
        let (subject, body) = {
            let conn = self.database.get_connection();
            let mut stmt = conn.prepare(
                "SELECT subject, body FROM email_templates WHERE id = ?1 AND user_id = ?2"
            )?;
            
            let (template_subject, template_body): (Option<String>, Option<String>) = stmt.query_row(
                [template_id, user_id],
                |row| Ok((row.get(0)?, row.get(1)?))
            )?;
            
            (template_subject.unwrap_or_default(), template_body.unwrap_or_default())
        };
        
        // Create personalization context
        let mut context = Context::new();
        context.insert("email", &recipient.email);
        
        // Add variables to context
        for (key, value) in &recipient.variables {
            context.insert(key, value);
        }
        
        // Personalize subject and body
        let mut tera = Tera::new("templates/**/*").unwrap_or_else(|_| Tera::new("").unwrap());
        
        let personalized_subject = tera.render_str(&subject, &context)
            .unwrap_or_else(|_| subject.clone());
        
        let personalized_body = tera.render_str(&body, &context)
            .unwrap_or_else(|_| body.clone());
        
        // Create email message
        let email_message = EmailMessage {
            to: vec![recipient.email.clone()],
            cc: None,
            bcc: None,
            subject: personalized_subject.clone(),
            body: personalized_body.clone(),
            attachments: None,
        };
        
        // Get user's email accounts (scope the connection)
        let (active_account, password) = {
            let email_accounts = self.database.get_email_accounts(user_id)
                .map_err(|e| AppError::Internal(e.to_string()))?;
            let active_account = email_accounts.into_iter()
                .find(|acc| acc.is_active)
                .ok_or_else(|| AppError::NotFound("No active email account found".to_string()))?;
            
            // TODO: Decrypt password properly
            let password = active_account.password_encrypted.clone();
            (active_account, password)
        };
        
        // Send the email
        let email_service = self.email_service.lock().await;
        email_service.send_email(&active_account, &password, &email_message).await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        
        // Log the sent email
        self.log_sent_email(
            user_id,
            &recipient.email,
            &personalized_subject,
            "sent",
            Some(campaign_id),
        )?;
        
        Ok(())
    }
    
    fn log_sent_email(
        &self,
        user_id: i32,
        recipient: &str,
        subject: &str,
        status: &str,
        campaign_id: Option<i32>,
    ) -> Result<(), AppError> {
        let conn = self.database.get_connection();
        
        conn.execute(
            "INSERT INTO email_logs (user_id, recipient_email, subject, status, campaign_id, sent_at)
             VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)",
            (
                user_id,
                recipient,
                subject,
                status,
                campaign_id,
            ),
        )?;
        
        Ok(())
    }
    
    fn log_email_failure(
        &self,
        user_id: i32,
        recipient: &str,
        subject: &str,
        error_message: &str,
    ) -> Result<(), AppError> {
        let conn = self.database.get_connection();
        
        conn.execute(
            "INSERT INTO email_logs (user_id, recipient_email, subject, status, error_message, sent_at)
             VALUES (?1, ?2, ?3, 'failed', ?4, CURRENT_TIMESTAMP)",
            (
                user_id,
                recipient,
                subject,
                error_message,
            ),
        )?;
        
        Ok(())
    }
    
    // Campaign Statistics
    pub fn get_campaign_stats(&self, user_id: i32, campaign_id: i32) -> Result<CampaignStats, AppError> {
        let conn = self.database.get_connection();
        
        // Get campaign basic info
        let campaign = self.get_campaign(user_id, campaign_id)?;
        
        // Get detailed stats from email logs
        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) as count
             FROM email_logs 
             WHERE user_id = ?1 AND campaign_id = ?2
             GROUP BY status"
        )?;
        
        let stats_iter = stmt.query_map([user_id, campaign_id], |row| {
            Ok((
                row.get::<_, String>(0)?, // status
                row.get::<_, i32>(1)?,     // count
            ))
        })?;
        
        let mut sent = 0;
        let mut failed = 0;
        let mut pending = 0;
        
        for stat_result in stats_iter {
            let (status, count) = stat_result?;
            match status.as_str() {
                "sent" => sent = count,
                "failed" => failed = count,
                "pending" => pending = count,
                _ => {}
            }
        }
        
        Ok(CampaignStats {
            campaign_id,
            total_recipients: campaign.total_recipients,
            sent_count: sent,
            failed_count: failed,
            pending_count: pending,
            success_rate: if sent + failed > 0 {
                (sent as f64 / (sent + failed) as f64 * 100.0) as f32
            } else {
                0.0
            },
            status: campaign.status.clone(),
            created_at: campaign.created_at.to_string(),
            completed_at: if campaign.status == "completed" || campaign.status == "partial" {
                Some(campaign.updated_at.to_string())
            } else {
                None
            },
        })
    }
    
    pub fn get_user_campaign_stats(&self, user_id: i32) -> Result<Vec<CampaignStats>, AppError> {
        let campaigns = self.get_user_campaigns(user_id)?;
        let mut stats = Vec::new();
        
        for campaign in campaigns {
            if let Ok(campaign_stats) = self.get_campaign_stats(user_id, campaign.id) {
                stats.push(campaign_stats);
            }
        }
        
        Ok(stats)
    }
    
    // Template-based campaigns
    pub fn create_campaign_from_template(
        &self,
        user_id: i32,
        template_id: i32,
        campaign_name: String,
        contact_list_id: Option<i32>,
    ) -> Result<EmailCampaign, AppError> {
        // Get template
        let conn = self.database.get_connection();
        let mut stmt = conn.prepare(
            "SELECT name, subject, body FROM email_templates WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let template_data = stmt.query_row([template_id, user_id], |row| {
            Ok((
                row.get::<_, String>(0)?, // name
                row.get::<_, Option<String>>(1)?, // subject
                row.get::<_, Option<String>>(2)?, // body
            ))
        })?;
        
        let (template_name, template_subject, template_body) = template_data;
        
        let campaign_data = CreateEmailCampaign {
            name: campaign_name,
            contact_list_id,
            template_id: Some(template_id),
            scheduled_time: None,
        };
        
        self.create_campaign(user_id, campaign_data)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CampaignStats {
    pub campaign_id: i32,
    pub total_recipients: i32,
    pub sent_count: i32,
    pub failed_count: i32,
    pub pending_count: i32,
    pub success_rate: f32,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
}