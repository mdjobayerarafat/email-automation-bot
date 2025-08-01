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
        let conn = self.database.get_connection()?;
        
        let mut stmt = conn.prepare(
            "INSERT INTO email_campaigns (user_id, name, subject, body, contact_list_id, template_id, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'draft')"
        )?;
        
        let campaign_id = stmt.insert((
            user_id,
            &campaign_data.name,
            &campaign_data.subject,
            &campaign_data.body,
            campaign_data.contact_list_id,
            campaign_data.template_id,
        ))?;
        
        self.get_campaign(user_id, campaign_id as i32)
    }
    
    pub fn get_campaign(&self, user_id: i32, campaign_id: i32) -> Result<EmailCampaign, AppError> {
        let conn = self.database.get_connection()?;
        
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
                subject: row.get(3)?,
                body: row.get(4)?,
                contact_list_id: row.get(5)?,
                template_id: row.get(6)?,
                status: row.get(7)?,
                sent_count: row.get(8)?,
                total_count: row.get(9)?,
                scheduled_at: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        
        Ok(campaign)
    }
    
    pub fn get_user_campaigns(&self, user_id: i32) -> Result<Vec<EmailCampaign>, AppError> {
        let conn = self.database.get_connection()?;
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, subject, body, contact_list_id, template_id, status, 
                    sent_count, total_count, scheduled_at, created_at, updated_at
             FROM email_campaigns WHERE user_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let campaign_iter = stmt.query_map([user_id], |row| {
            Ok(EmailCampaign {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                subject: row.get(3)?,
                body: row.get(4)?,
                contact_list_id: row.get(5)?,
                template_id: row.get(6)?,
                status: row.get(7)?,
                sent_count: row.get(8)?,
                total_count: row.get(9)?,
                scheduled_at: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        
        let mut campaigns = Vec::new();
        for campaign in campaign_iter {
            campaigns.push(campaign?);
        }
        
        Ok(campaigns)
    }
    
    pub fn update_campaign(&self, user_id: i32, campaign_id: i32, campaign_data: CreateEmailCampaign) -> Result<EmailCampaign, AppError> {
        let conn = self.database.get_connection()?;
        
        conn.execute(
            "UPDATE email_campaigns SET name = ?1, subject = ?2, body = ?3, contact_list_id = ?4, 
                    template_id = ?5, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?6 AND user_id = ?7 AND status = 'draft'",
            (
                &campaign_data.name,
                &campaign_data.subject,
                &campaign_data.body,
                campaign_data.contact_list_id,
                campaign_data.template_id,
                campaign_id,
                user_id,
            ),
        )?;
        
        self.get_campaign(user_id, campaign_id)
    }
    
    pub fn delete_campaign(&self, user_id: i32, campaign_id: i32) -> Result<(), AppError> {
        let conn = self.database.get_connection()?;
        
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
        let conn = self.database.get_connection()?;
        
        // Get recipients data
        let recipients = if let Some(contact_list_id) = request.contact_list_id {
            // Get contacts from contact list
            self.contact_service.get_contacts_by_list(user_id, contact_list_id)?
                .into_iter()
                .map(|contact| RecipientData {
                    email: contact.email,
                    name: Some(contact.name),
                    custom_fields: contact.custom_fields,
                })
                .collect()
        } else {
            // Use provided recipients
            request.recipients
        };
        
        if recipients.is_empty() {
            return Err(AppError::Validation("No recipients provided".to_string()));
        }
        
        // Create campaign record if not exists
        let campaign_id = if let Some(id) = request.campaign_id {
            id
        } else {
            let mut stmt = conn.prepare(
                "INSERT INTO email_campaigns (user_id, name, subject, body, status, total_count)
                 VALUES (?1, ?2, ?3, ?4, 'sending', ?5)"
            )?;
            
            stmt.insert((
                user_id,
                format!("Batch Email - {}", Utc::now().format("%Y-%m-%d %H:%M")),
                &request.subject,
                &request.body,
                recipients.len() as i32,
            ))? as i32
        };
        
        // Update campaign status and total count
        conn.execute(
            "UPDATE email_campaigns SET status = 'sending', total_count = ?1, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?2 AND user_id = ?3",
            [recipients.len() as i32, campaign_id, user_id],
        )?;
        
        let mut sent_count = 0;
        let mut failed_count = 0;
        
        // Send emails to each recipient
        for recipient in recipients {
            match self.send_personalized_email(user_id, &request, &recipient, campaign_id).await {
                Ok(_) => {
                    sent_count += 1;
                    info!("Email sent successfully to {}", recipient.email);
                },
                Err(e) => {
                    failed_count += 1;
                    error!("Failed to send email to {}: {}", recipient.email, e);
                    
                    // Log the failure
                    self.log_email_failure(user_id, &recipient.email, &request.subject, &e.to_string())?;
                }
            }
            
            // Update campaign progress
            conn.execute(
                "UPDATE email_campaigns SET sent_count = ?1, updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                [sent_count, campaign_id],
            )?;
            
            // Add a small delay to avoid overwhelming the SMTP server
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        // Update final campaign status
        let final_status = if failed_count == 0 { "completed" } else { "partial" };
        conn.execute(
            "UPDATE email_campaigns SET status = ?1, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?2",
            [final_status, campaign_id],
        )?;
        
        info!(
            "Batch email campaign {} completed: {} sent, {} failed",
            campaign_id, sent_count, failed_count
        );
        
        Ok(())
    }
    
    async fn send_personalized_email(
        &self,
        user_id: i32,
        request: &BatchEmailRequest,
        recipient: &RecipientData,
        campaign_id: i32,
    ) -> Result<(), AppError> {
        // Create personalization context
        let mut context = Context::new();
        context.insert("email", &recipient.email);
        
        if let Some(name) = &recipient.name {
            context.insert("name", name);
        }
        
        // Add custom fields to context
        if let Some(custom_fields) = &recipient.custom_fields {
            if let Ok(fields) = serde_json::from_str::<HashMap<String, String>>(custom_fields) {
                for (key, value) in fields {
                    context.insert(&key, &value);
                }
            }
        }
        
        // Personalize subject and body
        let mut tera = Tera::new("templates/**/*").unwrap_or_else(|_| Tera::new("").unwrap());
        
        let personalized_subject = tera.render_str(&request.subject, &context)
            .unwrap_or_else(|_| request.subject.clone());
        
        let personalized_body = tera.render_str(&request.body, &context)
            .unwrap_or_else(|_| request.body.clone());
        
        // Create email message
        let email_message = EmailMessage {
            to: vec![recipient.email.clone()],
            cc: request.cc.clone(),
            bcc: request.bcc.clone(),
            subject: personalized_subject.clone(),
            body: personalized_body.clone(),
            attachments: request.attachments.clone(),
        };
        
        // Send the email
        let email_service = self.email_service.lock().await;
        email_service.send_email(user_id, email_message).await?;
        
        // Log the sent email
        self.log_sent_email(
            user_id,
            &recipient.email,
            &personalized_subject,
            &personalized_body,
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
        body: &str,
        status: &str,
        campaign_id: Option<i32>,
    ) -> Result<(), AppError> {
        let conn = self.database.get_connection()?;
        
        conn.execute(
            "INSERT INTO email_logs (user_id, recipient, subject, body, status, campaign_id, sent_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, CURRENT_TIMESTAMP)",
            (
                user_id,
                recipient,
                subject,
                body,
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
        let conn = self.database.get_connection()?;
        
        conn.execute(
            "INSERT INTO email_logs (user_id, recipient, subject, body, status, error_message, sent_at)
             VALUES (?1, ?2, ?3, '', 'failed', ?4, CURRENT_TIMESTAMP)",
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
        let conn = self.database.get_connection()?;
        
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
            total_recipients: campaign.total_count.unwrap_or(0),
            sent_count: sent,
            failed_count: failed,
            pending_count: pending,
            success_rate: if sent + failed > 0 {
                (sent as f64 / (sent + failed) as f64 * 100.0) as f32
            } else {
                0.0
            },
            status: campaign.status,
            created_at: campaign.created_at,
            completed_at: if campaign.status == "completed" || campaign.status == "partial" {
                campaign.updated_at
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
        let conn = self.database.get_connection()?;
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
            subject: template_subject.unwrap_or_else(|| format!("Campaign from {}", template_name)),
            body: template_body.unwrap_or_default(),
            contact_list_id,
            template_id: Some(template_id),
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