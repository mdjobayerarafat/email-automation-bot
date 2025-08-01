use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use chrono::Utc;
use log::{info, error, warn};
use crate::models::*;
use crate::database::Database;

pub struct AttachmentService {
    database: std::sync::Arc<Database>,
    attachments_dir: PathBuf,
}

impl AttachmentService {
    pub fn new(database: std::sync::Arc<Database>, app_data_dir: &Path) -> Result<Self, AppError> {
        let attachments_dir = app_data_dir.join("attachments");
        
        // Create attachments directory if it doesn't exist
        if !attachments_dir.exists() {
            fs::create_dir_all(&attachments_dir)
                .map_err(|e| AppError::Internal(format!("Failed to create attachments directory: {}", e)))?;
        }
        
        Ok(Self {
            database,
            attachments_dir,
        })
    }
    
    pub fn save_attachment(
        &self,
        user_id: i32,
        email_log_id: i32,
        filename: &str,
        content: &[u8],
        mime_type: Option<String>,
        sender_email: Option<String>,
    ) -> Result<EmailAttachment, AppError> {
        // Generate unique filename to avoid conflicts
        let timestamp = Utc::now().timestamp();
        let unique_filename = format!("{}_{}", timestamp, filename);
        let file_path = self.attachments_dir.join(&unique_filename);
        
        // Write file to disk
        let mut file = fs::File::create(&file_path)
            .map_err(|e| AppError::Internal(format!("Failed to create attachment file: {}", e)))?;
        
        file.write_all(content)
            .map_err(|e| AppError::Internal(format!("Failed to write attachment content: {}", e)))?;
        
        // Determine category based on mime type
        let category = self.categorize_attachment(&mime_type);
        
        // Save attachment metadata to database
        let attachment_data = CreateEmailAttachment {
            user_id,
            email_log_id,
            filename: unique_filename.clone(),
            original_filename: filename.to_string(),
            file_path: file_path.to_string_lossy().to_string(),
            file_size: Some(content.len() as i64),
            mime_type,
            sender_email,
            received_at: Some(Utc::now()),
            category: Some(category),
        };
        
        self.create_attachment(attachment_data)
    }
    
    fn categorize_attachment(&self, mime_type: &Option<String>) -> String {
        match mime_type {
            Some(mime) => {
                if mime.starts_with("image/") {
                    "image".to_string()
                } else if mime.starts_with("application/pdf") || 
                         mime.starts_with("application/msword") ||
                         mime.starts_with("application/vnd.openxmlformats-officedocument") ||
                         mime.starts_with("text/") {
                    "document".to_string()
                } else if mime.starts_with("application/zip") ||
                         mime.starts_with("application/x-rar") ||
                         mime.starts_with("application/x-7z") {
                    "archive".to_string()
                } else {
                    "other".to_string()
                }
            },
            None => "other".to_string(),
        }
    }
    
    pub fn create_attachment(&self, attachment_data: CreateEmailAttachment) -> Result<EmailAttachment, AppError> {
        let conn = self.database.get_connection()?;
        
        let mut stmt = conn.prepare(
            "INSERT INTO email_attachments (
                user_id, email_log_id, filename, original_filename, file_path,
                file_size, mime_type, sender_email, received_at, category
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        )?;
        
        let attachment_id = stmt.insert((
            attachment_data.user_id,
            attachment_data.email_log_id,
            &attachment_data.filename,
            &attachment_data.original_filename,
            &attachment_data.file_path,
            attachment_data.file_size,
            &attachment_data.mime_type,
            &attachment_data.sender_email,
            attachment_data.received_at,
            &attachment_data.category,
        ))?;
        
        self.get_attachment(attachment_id as i32)
    }
    
    pub fn get_attachment(&self, attachment_id: i32) -> Result<EmailAttachment, AppError> {
        let conn = self.database.get_connection()?;
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, email_log_id, filename, original_filename, file_path,
                    file_size, mime_type, sender_email, received_at, category, created_at
             FROM email_attachments WHERE id = ?1"
        )?;
        
        let attachment = stmt.query_row([attachment_id], |row| {
            Ok(EmailAttachment {
                id: row.get(0)?,
                user_id: row.get(1)?,
                email_log_id: row.get(2)?,
                filename: row.get(3)?,
                original_filename: row.get(4)?,
                file_path: row.get(5)?,
                file_size: row.get(6)?,
                mime_type: row.get(7)?,
                sender_email: row.get(8)?,
                received_at: row.get(9)?,
                category: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?;
        
        Ok(attachment)
    }
    
    pub fn get_user_attachments(&self, user_id: i32, limit: Option<i32>) -> Result<Vec<EmailAttachment>, AppError> {
        let conn = self.database.get_connection()?;
        
        let query = if let Some(limit) = limit {
            format!(
                "SELECT id, user_id, email_log_id, filename, original_filename, file_path,
                        file_size, mime_type, sender_email, received_at, category, created_at
                 FROM email_attachments WHERE user_id = ?1
                 ORDER BY created_at DESC LIMIT {}",
                limit
            )
        } else {
            "SELECT id, user_id, email_log_id, filename, original_filename, file_path,
                    file_size, mime_type, sender_email, received_at, category, created_at
             FROM email_attachments WHERE user_id = ?1
             ORDER BY created_at DESC".to_string()
        };
        
        let mut stmt = conn.prepare(&query)?;
        let attachment_iter = stmt.query_map([user_id], |row| {
            Ok(EmailAttachment {
                id: row.get(0)?,
                user_id: row.get(1)?,
                email_log_id: row.get(2)?,
                filename: row.get(3)?,
                original_filename: row.get(4)?,
                file_path: row.get(5)?,
                file_size: row.get(6)?,
                mime_type: row.get(7)?,
                sender_email: row.get(8)?,
                received_at: row.get(9)?,
                category: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?;
        
        let mut attachments = Vec::new();
        for attachment in attachment_iter {
            attachments.push(attachment?);
        }
        
        Ok(attachments)
    }
    
    pub fn get_attachment_categories(&self, user_id: i32) -> Result<Vec<AttachmentCategory>, AppError> {
        let conn = self.database.get_connection()?;
        
        let mut stmt = conn.prepare(
            "SELECT category, COUNT(*) as count, COALESCE(SUM(file_size), 0) as total_size
             FROM email_attachments WHERE user_id = ?1
             GROUP BY category"
        )?;
        
        let category_iter = stmt.query_map([user_id], |row| {
            Ok(AttachmentCategory {
                category: row.get::<_, Option<String>>(0)?.unwrap_or_else(|| "other".to_string()),
                count: row.get(1)?,
                total_size: row.get(2)?,
            })
        })?;
        
        let mut categories = Vec::new();
        for category in category_iter {
            categories.push(category?);
        }
        
        Ok(categories)
    }
    
    pub fn delete_attachment(&self, user_id: i32, attachment_id: i32) -> Result<(), AppError> {
        // First get the attachment to get the file path
        let attachment = self.get_attachment(attachment_id)?;
        
        // Verify ownership
        if attachment.user_id != user_id {
            return Err(AppError::Auth("Unauthorized access to attachment".to_string()));
        }
        
        // Delete file from disk
        if let Err(e) = fs::remove_file(&attachment.file_path) {
            warn!("Failed to delete attachment file {}: {}", attachment.file_path, e);
        }
        
        // Delete from database
        let conn = self.database.get_connection()?;
        conn.execute(
            "DELETE FROM email_attachments WHERE id = ?1 AND user_id = ?2",
            [attachment_id, user_id],
        )?;
        
        info!("Deleted attachment {} for user {}", attachment_id, user_id);
        Ok(())
    }
    
    pub fn cleanup_orphaned_attachments(&self) -> Result<(), AppError> {
        let conn = self.database.get_connection()?;
        
        // Find attachments that reference non-existent email logs
        let mut stmt = conn.prepare(
            "SELECT a.id, a.file_path FROM email_attachments a
             LEFT JOIN email_logs e ON a.email_log_id = e.id
             WHERE e.id IS NULL"
        )?;
        
        let orphaned_iter = stmt.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        })?;
        
        let mut orphaned_attachments = Vec::new();
        for result in orphaned_iter {
            orphaned_attachments.push(result?);
        }
        
        // Delete orphaned attachments
        for (attachment_id, file_path) in orphaned_attachments {
            // Delete file from disk
            if let Err(e) = fs::remove_file(&file_path) {
                warn!("Failed to delete orphaned attachment file {}: {}", file_path, e);
            }
            
            // Delete from database
            conn.execute("DELETE FROM email_attachments WHERE id = ?1", [attachment_id])?;
            info!("Cleaned up orphaned attachment {}", attachment_id);
        }
        
        Ok(())
    }
}