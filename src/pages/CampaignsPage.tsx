import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../styles/CampaignsPage.css';

interface EmailCampaign {
  id: number;
  user_id: number;
  name: string;
  description?: string;
  contact_list_id: number;
  template_id: number;
  status: 'draft' | 'sending' | 'completed' | 'failed';
  total_recipients: number;
  sent_count: number;
  failed_count: number;
  scheduled_at?: string;
  created_at: string;
  updated_at: string;
}

interface ContactList {
  id: number;
  name: string;
  contact_count: number;
}

interface EmailTemplate {
  id: number;
  name: string;
  subject?: string;
}

interface CreateCampaign {
  name: string;
  description?: string;
  contact_list_id: number;
  template_id: number;
  scheduled_at?: string;
}

interface CampaignStats {
  total_sent: number;
  total_failed: number;
  delivery_rate: number;
}

interface CampaignsPageProps {
  token: string;
}

const CampaignsPage: React.FC<CampaignsPageProps> = ({ token }) => {
  const [campaigns, setCampaigns] = useState<EmailCampaign[]>([]);
  const [contactLists, setContactLists] = useState<ContactList[]>([]);
  const [templates, setTemplates] = useState<EmailTemplate[]>([]);
  const [selectedCampaign, setSelectedCampaign] = useState<EmailCampaign | null>(null);
  const [campaignStats, setCampaignStats] = useState<CampaignStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [sending, setSending] = useState<number | null>(null);
  
  const [newCampaign, setNewCampaign] = useState<CreateCampaign>({
    name: '',
    description: '',
    contact_list_id: 0,
    template_id: 0,
    scheduled_at: '',
  });

  useEffect(() => {
    loadData();
  }, [token]);

  const loadData = async () => {
    try {
      setLoading(true);
      const [campaignsData, listsData, templatesData] = await Promise.all([
        invoke<EmailCampaign[]>('get_campaigns', { token }),
        invoke<ContactList[]>('get_contact_lists', { token }),
        invoke<EmailTemplate[]>('get_email_templates', { token }),
      ]);
      
      setCampaigns(campaignsData);
      setContactLists(listsData);
      setTemplates(templatesData);
      setError(null);
    } catch (err) {
      setError(err as string);
      console.error('Failed to load campaigns data:', err);
    } finally {
      setLoading(false);
    }
  };

  const loadCampaignStats = async (campaignId: number) => {
    try {
      const stats = await invoke<CampaignStats>('get_campaign_stats', {
        token,
        campaignId,
      });
      setCampaignStats(stats);
    } catch (err) {
      console.error('Failed to load campaign stats:', err);
    }
  };

  const handleCreateCampaign = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke('create_campaign', {
        token,
        campaignData: {
          ...newCampaign,
          scheduled_at: newCampaign.scheduled_at || undefined,
        },
      });
      
      setShowCreateForm(false);
      setNewCampaign({
        name: '',
        description: '',
        contact_list_id: 0,
        template_id: 0,
        scheduled_at: '',
      });
      await loadData();
    } catch (err) {
      setError(err as string);
    }
  };

  const handleSendCampaign = async (campaignId: number) => {
    try {
      setSending(campaignId);
      await invoke('send_campaign', {
        token,
        campaignId,
      });
      
      // Reload campaigns to get updated status
      await loadData();
      
      // If this campaign is selected, reload its stats
      if (selectedCampaign?.id === campaignId) {
        await loadCampaignStats(campaignId);
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setSending(null);
    }
  };

  const handleSelectCampaign = async (campaign: EmailCampaign) => {
    setSelectedCampaign(campaign);
    await loadCampaignStats(campaign.id);
  };

  const getStatusColor = (status: string): string => {
    switch (status) {
      case 'draft': return '#6c757d';
      case 'sending': return '#ffc107';
      case 'completed': return '#28a745';
      case 'failed': return '#dc3545';
      default: return '#6c757d';
    }
  };

  const getStatusText = (status: string): string => {
    switch (status) {
      case 'draft': return 'Draft';
      case 'sending': return 'Sending';
      case 'completed': return 'Completed';
      case 'failed': return 'Failed';
      default: return status;
    }
  };

  const getContactListName = (listId: number): string => {
    const list = contactLists.find(l => l.id === listId);
    return list ? list.name : 'Unknown List';
  };

  const getTemplateName = (templateId: number): string => {
    const template = templates.find(t => t.id === templateId);
    return template ? template.name : 'Unknown Template';
  };

  const formatDateTime = (dateString: string): string => {
    return new Date(dateString).toLocaleString();
  };

  const calculateProgress = (campaign: EmailCampaign): number => {
    if (campaign.total_recipients === 0) return 0;
    return Math.round(((campaign.sent_count + campaign.failed_count) / campaign.total_recipients) * 100);
  };

  if (loading) {
    return (
      <div className="campaigns-page">
        <div className="loading">Loading campaigns...</div>
      </div>
    );
  }

  return (
    <div className="campaigns-page">
      <div className="page-header">
        <h1>Email Campaigns</h1>
        <button 
          className="btn btn-primary"
          onClick={() => setShowCreateForm(true)}
        >
          Create Campaign
        </button>
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      <div className="content-layout">
        <div className="campaigns-list">
          <h2>Campaigns</h2>
          {campaigns.length === 0 ? (
            <div className="empty-state">
              <p>No campaigns yet.</p>
              <p>Create your first campaign to start sending bulk emails.</p>
            </div>
          ) : (
            <div className="campaigns-container">
              {campaigns.map(campaign => (
                <div 
                  key={campaign.id} 
                  className={`campaign-card ${selectedCampaign?.id === campaign.id ? 'selected' : ''}`}
                  onClick={() => handleSelectCampaign(campaign)}
                >
                  <div className="campaign-header">
                    <div className="campaign-name">{campaign.name}</div>
                    <div 
                      className="campaign-status"
                      style={{ backgroundColor: getStatusColor(campaign.status) }}
                    >
                      {getStatusText(campaign.status)}
                    </div>
                  </div>
                  
                  <div className="campaign-info">
                    <div className="info-item">
                      <span className="label">List:</span>
                      <span className="value">{getContactListName(campaign.contact_list_id)}</span>
                    </div>
                    <div className="info-item">
                      <span className="label">Template:</span>
                      <span className="value">{getTemplateName(campaign.template_id)}</span>
                    </div>
                    <div className="info-item">
                      <span className="label">Recipients:</span>
                      <span className="value">{campaign.total_recipients}</span>
                    </div>
                  </div>
                  
                  {campaign.status === 'sending' && (
                    <div className="progress-bar">
                      <div 
                        className="progress-fill"
                        style={{ width: `${calculateProgress(campaign)}%` }}
                      ></div>
                      <span className="progress-text">
                        {calculateProgress(campaign)}% ({campaign.sent_count + campaign.failed_count}/{campaign.total_recipients})
                      </span>
                    </div>
                  )}
                  
                  <div className="campaign-actions">
                    {campaign.status === 'draft' && (
                      <button
                        className="btn btn-primary btn-sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleSendCampaign(campaign.id);
                        }}
                        disabled={sending === campaign.id}
                      >
                        {sending === campaign.id ? 'Sending...' : 'Send Now'}
                      </button>
                    )}
                    
                    <div className="campaign-date">
                      {formatDateTime(campaign.created_at)}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        <div className="campaign-details">
          {selectedCampaign ? (
            <>
              <div className="details-header">
                <h2>{selectedCampaign.name}</h2>
                <div 
                  className="status-badge"
                  style={{ backgroundColor: getStatusColor(selectedCampaign.status) }}
                >
                  {getStatusText(selectedCampaign.status)}
                </div>
              </div>
              
              {selectedCampaign.description && (
                <div className="campaign-description">
                  <h3>Description</h3>
                  <p>{selectedCampaign.description}</p>
                </div>
              )}
              
              <div className="campaign-metrics">
                <h3>Campaign Metrics</h3>
                <div className="metrics-grid">
                  <div className="metric-card">
                    <div className="metric-value">{selectedCampaign.total_recipients}</div>
                    <div className="metric-label">Total Recipients</div>
                  </div>
                  <div className="metric-card">
                    <div className="metric-value">{selectedCampaign.sent_count}</div>
                    <div className="metric-label">Sent</div>
                  </div>
                  <div className="metric-card">
                    <div className="metric-value">{selectedCampaign.failed_count}</div>
                    <div className="metric-label">Failed</div>
                  </div>
                  {campaignStats && (
                    <div className="metric-card">
                      <div className="metric-value">{campaignStats.delivery_rate.toFixed(1)}%</div>
                      <div className="metric-label">Delivery Rate</div>
                    </div>
                  )}
                </div>
              </div>
              
              <div className="campaign-config">
                <h3>Configuration</h3>
                <div className="config-grid">
                  <div className="config-item">
                    <span className="config-label">Contact List:</span>
                    <span className="config-value">{getContactListName(selectedCampaign.contact_list_id)}</span>
                  </div>
                  <div className="config-item">
                    <span className="config-label">Template:</span>
                    <span className="config-value">{getTemplateName(selectedCampaign.template_id)}</span>
                  </div>
                  <div className="config-item">
                    <span className="config-label">Created:</span>
                    <span className="config-value">{formatDateTime(selectedCampaign.created_at)}</span>
                  </div>
                  {selectedCampaign.scheduled_at && (
                    <div className="config-item">
                      <span className="config-label">Scheduled:</span>
                      <span className="config-value">{formatDateTime(selectedCampaign.scheduled_at)}</span>
                    </div>
                  )}
                </div>
              </div>
            </>
          ) : (
            <div className="no-selection">
              <h2>Select a Campaign</h2>
              <p>Choose a campaign from the list to view details and metrics.</p>
            </div>
          )}
        </div>
      </div>

      {showCreateForm && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <h2>Create Email Campaign</h2>
              <button 
                className="close-btn"
                onClick={() => setShowCreateForm(false)}
              >
                ×
              </button>
            </div>
            <form onSubmit={handleCreateCampaign}>
              <div className="form-group">
                <label>Campaign Name:</label>
                <input
                  type="text"
                  value={newCampaign.name}
                  onChange={(e) => setNewCampaign({ ...newCampaign, name: e.target.value })}
                  required
                  placeholder="Enter campaign name"
                />
              </div>
              
              <div className="form-group">
                <label>Description (optional):</label>
                <textarea
                  value={newCampaign.description}
                  onChange={(e) => setNewCampaign({ ...newCampaign, description: e.target.value })}
                  placeholder="Enter campaign description"
                  rows={3}
                />
              </div>
              
              <div className="form-group">
                <label>Contact List:</label>
                <select
                  value={newCampaign.contact_list_id}
                  onChange={(e) => setNewCampaign({ ...newCampaign, contact_list_id: parseInt(e.target.value) })}
                  required
                >
                  <option value={0}>Select a contact list</option>
                  {contactLists.map(list => (
                    <option key={list.id} value={list.id}>
                      {list.name} ({list.contact_count} contacts)
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="form-group">
                <label>Email Template:</label>
                <select
                  value={newCampaign.template_id}
                  onChange={(e) => setNewCampaign({ ...newCampaign, template_id: parseInt(e.target.value) })}
                  required
                >
                  <option value={0}>Select a template</option>
                  {templates.map(template => (
                    <option key={template.id} value={template.id}>
                      {template.name}
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="form-group">
                <label>Schedule (optional):</label>
                <input
                  type="datetime-local"
                  value={newCampaign.scheduled_at}
                  onChange={(e) => setNewCampaign({ ...newCampaign, scheduled_at: e.target.value })}
                />
                <small>Leave empty to send immediately</small>
              </div>
              
              <div className="form-actions">
                <button type="button" onClick={() => setShowCreateForm(false)}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create Campaign
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default CampaignsPage;