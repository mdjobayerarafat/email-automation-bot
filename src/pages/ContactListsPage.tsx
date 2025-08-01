import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './ContactListsPage.css';

interface ContactList {
  id: number;
  user_id: number;
  name: string;
  description?: string;
  contact_count: number;
  created_at: string;
  updated_at: string;
}

interface Contact {
  id: number;
  contact_list_id: number;
  email: string;
  first_name?: string;
  last_name?: string;
  custom_fields?: Record<string, any>;
  created_at: string;
}

interface CreateContactList {
  name: string;
  description?: string;
}

interface ImportContactsRequest {
  contact_list_id: number;
  csv_content: string;
  field_mapping: Record<string, string>;
}

interface ContactListsPageProps {
  token: string;
}

const ContactListsPage: React.FC<ContactListsPageProps> = ({ token }) => {
  const [contactLists, setContactLists] = useState<ContactList[]>([]);
  const [selectedList, setSelectedList] = useState<ContactList | null>(null);
  const [contacts, setContacts] = useState<Contact[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [showImportForm, setShowImportForm] = useState(false);
  
  const [newList, setNewList] = useState<CreateContactList>({
    name: '',
    description: '',
  });
  
  const [importData, setImportData] = useState({
    csvContent: '',
    fieldMapping: {
      email: 'email',
      first_name: 'first_name',
      last_name: 'last_name',
    },
  });
  
  const [csvHeaders, setCsvHeaders] = useState<string[]>([]);

  useEffect(() => {
    loadContactLists();
  }, [token]);

  const loadContactLists = async () => {
    try {
      setLoading(true);
      const lists = await invoke<ContactList[]>('get_contact_lists', { token });
      setContactLists(lists);
      setError(null);
    } catch (err) {
      setError(err as string);
      console.error('Failed to load contact lists:', err);
    } finally {
      setLoading(false);
    }
  };

  const loadContacts = async (listId: number) => {
    try {
      const contactsData = await invoke<Contact[]>('get_contacts', {
        token,
        contactListId: listId,
      });
      setContacts(contactsData);
    } catch (err) {
      setError(err as string);
      console.error('Failed to load contacts:', err);
    }
  };

  const handleCreateList = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke('create_contact_list', {
        token,
        listData: newList,
      });
      
      setShowCreateForm(false);
      setNewList({ name: '', description: '' });
      await loadContactLists();
    } catch (err) {
      setError(err as string);
    }
  };

  const handleSelectList = async (list: ContactList) => {
    setSelectedList(list);
    await loadContacts(list.id);
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const content = event.target?.result as string;
        setImportData({ ...importData, csvContent: content });
        
        // Parse headers from first line
        const lines = content.split('\n');
        if (lines.length > 0) {
          const headers = lines[0].split(',').map(h => h.trim().replace(/"/g, ''));
          setCsvHeaders(headers);
        }
      };
      reader.readAsText(file);
    }
  };

  const handleImportContacts = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedList) return;
    
    try {
      const importRequest: ImportContactsRequest = {
        contact_list_id: selectedList.id,
        csv_content: importData.csvContent,
        field_mapping: importData.fieldMapping,
      };
      
      await invoke('import_contacts', {
        token,
        importData: importRequest,
      });
      
      setShowImportForm(false);
      setImportData({
        csvContent: '',
        fieldMapping: {
          email: 'email',
          first_name: 'first_name',
          last_name: 'last_name',
        },
      });
      setCsvHeaders([]);
      
      // Reload data
      await loadContactLists();
      await loadContacts(selectedList.id);
    } catch (err) {
      setError(err as string);
    }
  };

  const handleFieldMappingChange = (field: string, csvColumn: string) => {
    setImportData({
      ...importData,
      fieldMapping: {
        ...importData.fieldMapping,
        [field]: csvColumn,
      },
    });
  };

  if (loading) {
    return (
      <div className="contact-lists-page">
        <div className="loading">Loading contact lists...</div>
      </div>
    );
  }

  return (
    <div className="contact-lists-page">
      <div className="page-header">
        <h1>Contact Lists</h1>
        <button 
          className="btn btn-primary"
          onClick={() => setShowCreateForm(true)}
        >
          Create List
        </button>
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      <div className="content-layout">
        <div className="lists-sidebar">
          <h2>Lists</h2>
          {contactLists.length === 0 ? (
            <div className="empty-state">
              <p>No contact lists yet.</p>
              <p>Create your first list to get started.</p>
            </div>
          ) : (
            <div className="lists-container">
              {contactLists.map(list => (
                <div 
                  key={list.id} 
                  className={`list-item ${selectedList?.id === list.id ? 'selected' : ''}`}
                  onClick={() => handleSelectList(list)}
                >
                  <div className="list-name">{list.name}</div>
                  <div className="list-info">
                    <span className="contact-count">{list.contact_count} contacts</span>
                    <span className="created-date">
                      {new Date(list.created_at).toLocaleDateString()}
                    </span>
                  </div>
                  {list.description && (
                    <div className="list-description">{list.description}</div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>

        <div className="contacts-main">
          {selectedList ? (
            <>
              <div className="contacts-header">
                <h2>{selectedList.name}</h2>
                <button 
                  className="btn btn-secondary"
                  onClick={() => setShowImportForm(true)}
                >
                  Import CSV
                </button>
              </div>
              
              {contacts.length === 0 ? (
                <div className="empty-state">
                  <p>No contacts in this list.</p>
                  <p>Import contacts from a CSV file to get started.</p>
                </div>
              ) : (
                <div className="contacts-table">
                  <table>
                    <thead>
                      <tr>
                        <th>Email</th>
                        <th>First Name</th>
                        <th>Last Name</th>
                        <th>Added</th>
                      </tr>
                    </thead>
                    <tbody>
                      {contacts.map(contact => (
                        <tr key={contact.id}>
                          <td>{contact.email}</td>
                          <td>{contact.first_name || '-'}</td>
                          <td>{contact.last_name || '-'}</td>
                          <td>{new Date(contact.created_at).toLocaleDateString()}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </>
          ) : (
            <div className="no-selection">
              <h2>Select a Contact List</h2>
              <p>Choose a contact list from the sidebar to view and manage contacts.</p>
            </div>
          )}
        </div>
      </div>

      {showCreateForm && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <h2>Create Contact List</h2>
              <button 
                className="close-btn"
                onClick={() => setShowCreateForm(false)}
              >
                ×
              </button>
            </div>
            <form onSubmit={handleCreateList}>
              <div className="form-group">
                <label>List Name:</label>
                <input
                  type="text"
                  value={newList.name}
                  onChange={(e) => setNewList({ ...newList, name: e.target.value })}
                  required
                  placeholder="Enter list name"
                />
              </div>
              
              <div className="form-group">
                <label>Description (optional):</label>
                <textarea
                  value={newList.description}
                  onChange={(e) => setNewList({ ...newList, description: e.target.value })}
                  placeholder="Enter list description"
                  rows={3}
                />
              </div>
              
              <div className="form-actions">
                <button type="button" onClick={() => setShowCreateForm(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create List
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showImportForm && selectedList && (
        <div className="modal-overlay">
          <div className="modal large">
            <div className="modal-header">
              <h2>Import Contacts to {selectedList.name}</h2>
              <button 
                className="close-btn"
                onClick={() => setShowImportForm(false)}
              >
                ×
              </button>
            </div>
            <form onSubmit={handleImportContacts}>
              <div className="form-group">
                <label>CSV File:</label>
                <input
                  type="file"
                  accept=".csv"
                  onChange={handleFileUpload}
                  required
                />
                <small>Upload a CSV file with contact information</small>
              </div>
              
              {csvHeaders.length > 0 && (
                <div className="form-group">
                  <label>Field Mapping:</label>
                  <div className="field-mapping">
                    <div className="mapping-row">
                      <span>Email (required):</span>
                      <select
                        value={importData.fieldMapping.email}
                        onChange={(e) => handleFieldMappingChange('email', e.target.value)}
                        required
                      >
                        <option value="">Select CSV column</option>
                        {csvHeaders.map(header => (
                          <option key={header} value={header}>{header}</option>
                        ))}
                      </select>
                    </div>
                    
                    <div className="mapping-row">
                      <span>First Name:</span>
                      <select
                        value={importData.fieldMapping.first_name}
                        onChange={(e) => handleFieldMappingChange('first_name', e.target.value)}
                      >
                        <option value="">Select CSV column (optional)</option>
                        {csvHeaders.map(header => (
                          <option key={header} value={header}>{header}</option>
                        ))}
                      </select>
                    </div>
                    
                    <div className="mapping-row">
                      <span>Last Name:</span>
                      <select
                        value={importData.fieldMapping.last_name}
                        onChange={(e) => handleFieldMappingChange('last_name', e.target.value)}
                      >
                        <option value="">Select CSV column (optional)</option>
                        {csvHeaders.map(header => (
                          <option key={header} value={header}>{header}</option>
                        ))}
                      </select>
                    </div>
                  </div>
                </div>
              )}
              
              <div className="form-actions">
                <button type="button" onClick={() => setShowImportForm(false)}>
                  Cancel
                </button>
                <button 
                  type="submit" 
                  className="btn btn-primary"
                  disabled={!importData.csvContent || !importData.fieldMapping.email}
                >
                  Import Contacts
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default ContactListsPage;