import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { EmailAccount, EmailTemplate } from '../types';
import '../styles/InboxMonitorPage.css';

interface InboxMonitor {
  id: number;
  user_id: number;
  email_account_id: number;
  is_active: boolean;
  check_interval: number;
  last_check?: string;
  auto_reply_template_id?: number;
  created_at: string;
}

interface CreateInboxMonitor {
  email_account_id: number;
  check_interval?: number;
  auto_reply_template_id?: number;
}

interface InboxEmail {
  id: string;
  subject: string;
  sender: string;
  received_at: string;
  body: string;
  attachments: any[];
  is_read: boolean;
}

interface InboxMonitorPageProps {
  token: string;
}

const InboxMonitorPage: React.FC<InboxMonitorPageProps> = ({ token }) => {
  const [monitors, setMonitors] = useState<InboxMonitor[]>([]);
  const [emailAccounts, setEmailAccounts] = useState<EmailAccount[]>([]);
  const [templates, setTemplates] = useState<EmailTemplate[]>([]);
  const [recentEmails, setRecentEmails] = useState<InboxEmail[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [selectedAccountId, setSelectedAccountId] = useState<number | null>(null);
  
  const [newMonitor, setNewMonitor] = useState<CreateInboxMonitor>({
    email_account_id: 0,
    check_interval: 300, // 5 minutes
    auto_reply_template_id: undefined,
  });

  useEffect(() => {
    loadData();
  }, [token]);

  const loadData = async () => {
    try {
      setLoading(true);
      const [monitorsData, accountsData, templatesData] = await Promise.all([
        invoke<InboxMonitor[]>('get_inbox_monitors', { token }),
        invoke<EmailAccount[]>('get_email_accounts', { token }),
        invoke<EmailTemplate[]>('get_email_templates', { token }),
      ]);
      
      setMonitors(monitorsData);
      setEmailAccounts(accountsData);
      setTemplates(templatesData);
      setError(null);
    } catch (err) {
      setError(err as string);
      console.error('Failed to load inbox monitor data:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateMonitor = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke('create_inbox_monitor', {
        token,
        monitorData: newMonitor,
      });
      
      setShowCreateForm(false);
      setNewMonitor({
        email_account_id: 0,
        check_interval: 300,
        auto_reply_template_id: undefined,
      });
      await loadData();
    } catch (err) {
      setError(err as string);
    }
  };

  const handleToggleMonitor = async (monitorId: number, isActive: boolean) => {
    try {
      await invoke('toggle_inbox_monitor', {
        token,
        monitorId,
        isActive: !isActive,
      });
      await loadData();
    } catch (err) {
      setError(err as string);
    }
  };

  const handleCheckInbox = async (accountId: number) => {
    try {
      setSelectedAccountId(accountId);
      const emails = await invoke<InboxEmail[]>('check_inbox', {
        token,
        accountId,
      });
      setRecentEmails(emails);
    } catch (err) {
      setError(err as string);
    }
  };

  const formatInterval = (seconds: number): string => {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
    return `${Math.floor(seconds / 3600)}h`;
  };

  const getAccountName = (accountId: number): string => {
    const account = emailAccounts.find(acc => acc.id === accountId);
    return account ? account.email_address : 'Unknown Account';
  };

  const getTemplateName = (templateId?: number): string => {
    if (!templateId) return 'No auto-reply';
    const template = templates.find(t => t.id === templateId);
    return template ? template.name : 'Unknown Template';
  };

  if (loading) {
    return (
      <div className="inbox-monitor-page">
        <div className="loading">Loading inbox monitors...</div>
      </div>
    );
  }

  return (
    <div className="inbox-monitor-page">
      <div className="page-header">
        <h1>Inbox Monitor</h1>
        <button 
          className="btn btn-primary"
          onClick={() => setShowCreateForm(true)}
        >
          Add Monitor
        </button>
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      {showCreateForm && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <h2>Create Inbox Monitor</h2>
              <button 
                className="close-btn"
                onClick={() => setShowCreateForm(false)}
              >
                ×
              </button>
            </div>
            <form onSubmit={handleCreateMonitor}>
              <div className="form-group">
                <label>Email Account:</label>
                <select
                  value={newMonitor.email_account_id}
                  onChange={(e) => setNewMonitor({
                    ...newMonitor,
                    email_account_id: parseInt(e.target.value)
                  })}
                  required
                >
                  <option value={0}>Select an account</option>
                  {emailAccounts.map(account => (
                    <option key={account.id} value={account.id}>
                      {account.email_address}
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="form-group">
                <label>Check Interval (seconds):</label>
                <input
                  type="number"
                  min="60"
                  value={newMonitor.check_interval}
                  onChange={(e) => setNewMonitor({
                    ...newMonitor,
                    check_interval: parseInt(e.target.value)
                  })}
                  required
                />
              </div>
              
              <div className="form-group">
                <label>Auto-Reply Template (optional):</label>
                <select
                  value={newMonitor.auto_reply_template_id || ''}
                  onChange={(e) => setNewMonitor({
                    ...newMonitor,
                    auto_reply_template_id: e.target.value ? parseInt(e.target.value) : undefined
                  })}
                >
                  <option value="">No auto-reply</option>
                  {templates.map(template => (
                    <option key={template.id} value={template.id}>
                      {template.name}
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="form-actions">
                <button type="button" onClick={() => setShowCreateForm(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create Monitor
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      <div className="monitors-section">
        <h2>Active Monitors</h2>
        {monitors.length === 0 ? (
          <div className="empty-state">
            <p>No inbox monitors configured.</p>
            <p>Create your first monitor to start monitoring incoming emails.</p>
          </div>
        ) : (
          <div className="monitors-grid">
            {monitors.map(monitor => (
              <div key={monitor.id} className={`monitor-card ${monitor.is_active ? 'active' : 'inactive'}`}>
                <div className="monitor-header">
                  <h3>{getAccountName(monitor.email_account_id)}</h3>
                  <div className="monitor-status">
                    <span className={`status-indicator ${monitor.is_active ? 'active' : 'inactive'}`}>
                      {monitor.is_active ? 'Active' : 'Inactive'}
                    </span>
                  </div>
                </div>
                
                <div className="monitor-details">
                  <div className="detail-item">
                    <span className="label">Check Interval:</span>
                    <span className="value">{formatInterval(monitor.check_interval)}</span>
                  </div>
                  
                  <div className="detail-item">
                    <span className="label">Auto-Reply:</span>
                    <span className="value">{getTemplateName(monitor.auto_reply_template_id)}</span>
                  </div>
                  
                  {monitor.last_check && (
                    <div className="detail-item">
                      <span className="label">Last Check:</span>
                      <span className="value">
                        {new Date(monitor.last_check).toLocaleString()}
                      </span>
                    </div>
                  )}
                </div>
                
                <div className="monitor-actions">
                  <button
                    className={`btn ${monitor.is_active ? 'btn-secondary' : 'btn-primary'}`}
                    onClick={() => handleToggleMonitor(monitor.id, monitor.is_active)}
                  >
                    {monitor.is_active ? 'Pause' : 'Start'}
                  </button>
                  
                  <button
                    className="btn btn-outline"
                    onClick={() => handleCheckInbox(monitor.email_account_id)}
                  >
                    Check Now
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {recentEmails.length > 0 && (
        <div className="recent-emails-section">
          <h2>Recent Emails ({getAccountName(selectedAccountId!)})</h2>
          <div className="emails-list">
            {recentEmails.map(email => (
              <div key={email.id} className="email-item">
                <div className="email-header">
                  <div className="email-subject">{email.subject}</div>
                  <div className="email-date">
                    {new Date(email.received_at).toLocaleString()}
                  </div>
                </div>
                <div className="email-sender">From: {email.sender}</div>
                <div className="email-preview">
                  {email.body.substring(0, 200)}...
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default InboxMonitorPage;