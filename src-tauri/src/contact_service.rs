use std::collections::HashMap;
use chrono::Utc;
use log::{info, error};
use csv::ReaderBuilder;
use crate::models::*;
use crate::database::Database;

pub struct ContactService {
    database: std::sync::Arc<Database>,
}

impl ContactService {
    pub fn new(database: std::sync::Arc<Database>) -> Self {
        Self { database }
    }
    
    // Contact List Management
    pub fn create_contact_list(&self, user_id: i32, list_data: CreateContactList) -> Result<ContactList, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "INSERT INTO contact_lists (user_id, name, description) VALUES (?1, ?2, ?3)"
        )?;
        
        let list_id = stmt.insert((
            user_id,
            &list_data.name,
            &list_data.description,
        ))?;
        
        self.get_contact_list(user_id, list_id as i32)
    }
    
    pub fn get_contact_list(&self, user_id: i32, list_id: i32) -> Result<ContactList, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, description, created_at, updated_at
             FROM contact_lists WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let list = stmt.query_row([list_id, user_id], |row| {
            Ok(ContactList {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        
        Ok(list)
    }
    
    pub fn get_user_contact_lists(&self, user_id: i32) -> Result<Vec<ContactList>, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, description, created_at, updated_at
             FROM contact_lists WHERE user_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let list_iter = stmt.query_map([user_id], |row| {
            Ok(ContactList {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        
        let mut lists = Vec::new();
        for list in list_iter {
            lists.push(list?);
        }
        
        Ok(lists)
    }
    
    pub fn update_contact_list(&self, user_id: i32, list_id: i32, list_data: CreateContactList) -> Result<ContactList, AppError> {
        let conn = self.database.get_connection();
        
        conn.execute(
            "UPDATE contact_lists SET name = ?1, description = ?2, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?3 AND user_id = ?4",
            (&list_data.name, &list_data.description, list_id, user_id),
        )?;
        
        self.get_contact_list(user_id, list_id)
    }
    
    pub fn delete_contact_list(&self, user_id: i32, list_id: i32) -> Result<(), AppError> {
        let conn = self.database.get_connection();
        
        // First delete all contacts in the list
        conn.execute(
            "DELETE FROM contacts WHERE contact_list_id = ?1 AND user_id = ?2",
            [list_id, user_id],
        )?;
        
        // Then delete the list
        let rows_affected = conn.execute(
            "DELETE FROM contact_lists WHERE id = ?1 AND user_id = ?2",
            [list_id, user_id],
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound("Contact list not found".to_string()));
        }
        
        info!("Deleted contact list {} for user {}", list_id, user_id);
        Ok(())
    }
    
    // Contact Management
    pub fn create_contact(&self, user_id: i32, contact_data: CreateContact) -> Result<Contact, AppError> {
        let conn = self.database.get_connection();
        
        // Verify the contact list belongs to the user
        let list_exists = conn.query_row(
            "SELECT 1 FROM contact_lists WHERE id = ?1 AND user_id = ?2",
            [contact_data.contact_list_id, user_id],
            |_| Ok(())
        );
        
        if list_exists.is_err() {
            return Err(AppError::NotFound("Contact list not found".to_string()));
        }
        
        let custom_fields_json = contact_data.custom_fields
            .as_ref()
            .map(|cf| serde_json::to_string(cf))
            .transpose()
            .map_err(|e| AppError::Internal(format!("Failed to serialize custom fields: {}", e)))?;
        
        let mut stmt = conn.prepare(
            "INSERT INTO contacts (user_id, contact_list_id, email, first_name, last_name, custom_fields)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )?;
        
        let contact_id = stmt.insert((
            user_id,
            contact_data.contact_list_id,
            &contact_data.email,
            &contact_data.first_name,
            &contact_data.last_name,
            &custom_fields_json,
        ))?;
        
        self.get_contact(user_id, contact_id as i32)
    }
    
    pub fn get_contact(&self, user_id: i32, contact_id: i32) -> Result<Contact, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, contact_list_id, email, first_name, last_name, custom_fields, is_active, created_at, updated_at
             FROM contacts WHERE id = ?1 AND user_id = ?2"
        )?;
        
        let contact = stmt.query_row([contact_id, user_id], |row| {
            let custom_fields_str: Option<String> = row.get(6)?;
            let custom_fields = custom_fields_str
                .as_ref()
                .map(|s| serde_json::from_str(s))
                .transpose()
                .map_err(|e| rusqlite::Error::InvalidColumnType(6, "custom_fields".to_string(), rusqlite::types::Type::Text))?;
            
            Ok(Contact {
                id: row.get(0)?,
                user_id: row.get(1)?,
                contact_list_id: row.get(2)?,
                email: row.get(3)?,
                first_name: row.get(4)?,
                last_name: row.get(5)?,
                custom_fields,
                is_active: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        
        Ok(contact)
    }
    
    pub fn get_contacts_by_list(&self, user_id: i32, list_id: i32) -> Result<Vec<Contact>, AppError> {
        let conn = self.database.get_connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, contact_list_id, email, first_name, last_name, custom_fields, is_active, created_at, updated_at
             FROM contacts WHERE contact_list_id = ?1 AND user_id = ?2 ORDER BY created_at DESC"
        )?;
        
        let contact_iter = stmt.query_map([list_id, user_id], |row| {
            let custom_fields_str: Option<String> = row.get(6)?;
            let custom_fields = custom_fields_str
                .as_ref()
                .map(|s| serde_json::from_str(s))
                .transpose()
                .map_err(|e| rusqlite::Error::InvalidColumnType(6, "custom_fields".to_string(), rusqlite::types::Type::Text))?;
            
            Ok(Contact {
                id: row.get(0)?,
                user_id: row.get(1)?,
                contact_list_id: row.get(2)?,
                email: row.get(3)?,
                first_name: row.get(4)?,
                last_name: row.get(5)?,
                custom_fields,
                is_active: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        
        let mut contacts = Vec::new();
        for contact in contact_iter {
            contacts.push(contact?);
        }
        
        Ok(contacts)
    }
    
    pub fn update_contact(&self, user_id: i32, contact_id: i32, contact_data: CreateContact) -> Result<Contact, AppError> {
        let conn = self.database.get_connection();
        
        let custom_fields_json = contact_data.custom_fields
            .as_ref()
            .map(|cf| serde_json::to_string(cf))
            .transpose()
            .map_err(|e| AppError::Internal(format!("Failed to serialize custom fields: {}", e)))?;
        
        conn.execute(
            "UPDATE contacts SET contact_list_id = ?1, email = ?2, first_name = ?3, last_name = ?4, custom_fields = ?5, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?6 AND user_id = ?7",
            (
                contact_data.contact_list_id,
                &contact_data.email,
                &contact_data.first_name,
                &contact_data.last_name,
                &custom_fields_json,
                contact_id,
                user_id,
            ),
        )?;
        
        self.get_contact(user_id, contact_id)
    }
    
    pub fn delete_contact(&self, user_id: i32, contact_id: i32) -> Result<(), AppError> {
        let conn = self.database.get_connection();
        
        let rows_affected = conn.execute(
            "DELETE FROM contacts WHERE id = ?1 AND user_id = ?2",
            [contact_id, user_id],
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound("Contact not found".to_string()));
        }
        
        info!("Deleted contact {} for user {}", contact_id, user_id);
        Ok(())
    }
    
    // CSV Import functionality
    pub fn import_contacts_from_csv(&self, user_id: i32, import_request: ImportContactsRequest) -> Result<Vec<Contact>, AppError> {
        // Verify the contact list belongs to the user
        let conn = self.database.get_connection();
        let list_exists = conn.query_row(
            "SELECT 1 FROM contact_lists WHERE id = ?1 AND user_id = ?2",
            [import_request.contact_list_id, user_id],
            |_| Ok(())
        );
        
        if list_exists.is_err() {
            return Err(AppError::NotFound("Contact list not found".to_string()));
        }
        
        // Parse CSV data
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(import_request.csv_data.as_bytes());
        
        let headers = reader.headers()
            .map_err(|e| AppError::Validation(format!("Invalid CSV format: {}", e)))?
            .clone();
        
        // Find required columns
        let email_col = headers.iter().position(|h| h.to_lowercase() == "email")
            .ok_or_else(|| AppError::Validation("CSV must contain an 'email' column".to_string()))?;
        
        let first_name_col = headers.iter().position(|h| h.to_lowercase() == "first_name" || h.to_lowercase() == "firstname");
        let last_name_col = headers.iter().position(|h| h.to_lowercase() == "last_name" || h.to_lowercase() == "lastname");
        
        let mut imported_contacts = Vec::new();
        let mut errors = Vec::new();
        
        for (line_num, result) in reader.records().enumerate() {
            match result {
                Ok(record) => {
                    let email = record.get(email_col)
                        .ok_or_else(|| AppError::Validation(format!("Line {}: Missing email", line_num + 2)))?;
                    
                    if email.trim().is_empty() {
                        errors.push(format!("Line {}: Empty email", line_num + 2));
                        continue;
                    }
                    
                    // Validate email format
                    if !email.contains('@') {
                        errors.push(format!("Line {}: Invalid email format", line_num + 2));
                        continue;
                    }
                    
                    let first_name = first_name_col
                        .and_then(|col| record.get(col))
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.to_string());
                    
                    let last_name = last_name_col
                        .and_then(|col| record.get(col))
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.to_string());
                    
                    // Collect custom fields (all columns except email, first_name, last_name)
                    let mut custom_fields = HashMap::new();
                    for (i, value) in record.iter().enumerate() {
                        if i != email_col && Some(i) != first_name_col && Some(i) != last_name_col {
                            if let Some(header) = headers.get(i) {
                                if !value.trim().is_empty() {
                                    custom_fields.insert(header.to_string(), value.to_string());
                                }
                            }
                        }
                    }
                    
                    let custom_fields_value = if custom_fields.is_empty() {
                        None
                    } else {
                        Some(serde_json::to_value(custom_fields)
                            .map_err(|e| AppError::Internal(format!("Failed to serialize custom fields: {}", e)))?)
                    };
                    
                    let contact_data = CreateContact {
                        contact_list_id: import_request.contact_list_id,
                        email: email.trim().to_string(),
                        first_name,
                        last_name,
                        custom_fields: custom_fields_value,
                    };
                    
                    match self.create_contact(user_id, contact_data) {
                        Ok(contact) => imported_contacts.push(contact),
                        Err(e) => errors.push(format!("Line {}: {}", line_num + 2, e)),
                    }
                },
                Err(e) => {
                    errors.push(format!("Line {}: CSV parsing error: {}", line_num + 2, e));
                }
            }
        }
        
        if !errors.is_empty() {
            error!("CSV import errors: {:?}", errors);
            // You might want to return partial success with errors
            // For now, we'll continue and return successful imports
        }
        
        info!("Imported {} contacts for user {} into list {}", imported_contacts.len(), user_id, import_request.contact_list_id);
        Ok(imported_contacts)
    }
    
    pub fn get_total_contacts_count(&self, user_id: i32) -> Result<i32, AppError> {
        let conn = self.database.get_connection();
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM contacts WHERE user_id = ?1 AND is_active = 1",
            [user_id],
            |row| row.get(0),
        )?;
        
        Ok(count)
    }
}