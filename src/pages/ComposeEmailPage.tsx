import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { EmailForm, EmailAccount, EmailTemplate } from '../types';
import { getPremadeTemplates, PremadeTemplate } from '../services/templateService';
import '../styles/ComposeEmailPage.css';

interface ComposeEmailPageProps {
  emailAccounts: EmailAccount[];
  isLoading: boolean;
  onSendEmail: (emailData: EmailForm) => Promise<void>;
  onError: (message: string) => void;
  onSuccess: (message: string) => void;
  templateData?: { subject?: string; body?: string } | null;
  onClearTemplate?: () => void;
}

const ComposeEmailPage: React.FC<ComposeEmailPageProps> = ({
  emailAccounts,
  isLoading,
  onSendEmail,
  onError,
  onSuccess,
  templateData,
  onClearTemplate
}) => {
  const [emailForm, setEmailForm] = useState<EmailForm>({
    to: [],
    subject: '',
    body: ''
  });
  
  const [templates, setTemplates] = useState<EmailTemplate[]>([]);
  const [premadeTemplates] = useState<PremadeTemplate[]>(getPremadeTemplates());
  const [selectedTemplate, setSelectedTemplate] = useState<string>('');
  const [showTemplateSelector, setShowTemplateSelector] = useState(false);
  
  // Fetch custom templates
  const fetchTemplates = async () => {
    try {
      const result = await invoke<EmailTemplate[]>('get_email_templates', {
        token: localStorage.getItem('auth_token')
      });
      setTemplates(result);
    } catch (error) {
      console.error('Error fetching templates:', error);
    }
  };
  
  useEffect(() => {
    fetchTemplates();
  }, []);
  
  // Apply template data from navigation
  useEffect(() => {
    if (templateData) {
      setEmailForm({
        ...emailForm,
        subject: templateData.subject || '',
        body: templateData.body || ''
      });
      setSelectedTemplate('from-navigation');
    }
  }, [templateData]);
  
  const handleTemplateSelect = (templateId: string) => {
    if (templateId === '') {
      setSelectedTemplate('');
      return;
    }
    
    // Check if it's a premade template (starts with premade-)
    if (templateId.startsWith('premade-')) {
      const premadeId = templateId.replace('premade-', '');
      const template = premadeTemplates.find(t => t.id === premadeId);
      if (template) {
        setEmailForm({
          ...emailForm,
          subject: template.subject,
          body: template.html_content
        });
      }
    } else {
      // Custom template
      const template = templates.find(t => t.id.toString() === templateId);
      if (template) {
        setEmailForm({
          ...emailForm,
          subject: template.subject || '',
          body: template.body || ''
        });
      }
    }
    
    setSelectedTemplate(templateId);
    setShowTemplateSelector(false);
  };
  
  const clearTemplate = () => {
    setEmailForm({
      to: [],
      subject: '',
      body: ''
    });
    setSelectedTemplate('');
    if (onClearTemplate) {
      onClearTemplate();
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    try {
      await onSendEmail(emailForm);
      onSuccess('Email sent successfully!');
      
      // Reset form
      setEmailForm({
        to: [],
        subject: '',
        body: ''
      });
      setSelectedTemplate('');
    } catch (error) {
      onError(error instanceof Error ? error.message : 'Failed to send email');
    }
  };

  return (
    <div className="compose-email-page">
      <h2>Compose Email</h2>
      
      {/* Template Selection */}
       <div className="template-section">
         <div className="template-controls">
           <button
             type="button"
             onClick={() => setShowTemplateSelector(!showTemplateSelector)}
             className="template-toggle-btn"
           >
             {showTemplateSelector ? 'Hide Templates' : 'Use Template'}
           </button>
           {selectedTemplate && (
             <button
               type="button"
               onClick={clearTemplate}
               className="clear-template-btn"
             >
               Clear Template
             </button>
           )}
           {templateData && (
             <span className="template-indicator">
               ✓ Template loaded from Templates page
             </span>
           )}
         </div>
        
        {showTemplateSelector && (
          <div className="template-selector">
            <select
              value={selectedTemplate}
              onChange={(e) => handleTemplateSelect(e.target.value)}
              className="template-select"
            >
              <option value="">Select a template...</option>
              <optgroup label="Pre-made Templates">
                {premadeTemplates.map(template => (
                  <option key={`premade-${template.id}`} value={`premade-${template.id}`}>
                    {template.name}
                  </option>
                ))}
              </optgroup>
              {templates.length > 0 && (
                <optgroup label="Custom Templates">
                  {templates.map(template => (
                    <option key={template.id} value={template.id.toString()}>
                      {template.name}
                    </option>
                  ))}
                </optgroup>
              )}
            </select>
          </div>
        )}
      </div>
      
      <form onSubmit={handleSubmit} className="email-form">
        <div className="form-group">
          <label htmlFor="to">To:</label>
          <input
            type="text"
            id="to"
            placeholder="To (comma-separated emails)"
            value={emailForm.to.join(', ')}
            onChange={(e) => setEmailForm({...emailForm, to: e.target.value.split(',').map(email => email.trim()).filter(email => email)})}
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="cc">CC (optional):</label>
          <input
            type="text"
            id="cc"
            placeholder="CC (comma-separated emails)"
            value={emailForm.cc?.join(', ') || ''}
            onChange={(e) => setEmailForm({...emailForm, cc: e.target.value ? e.target.value.split(',').map(email => email.trim()).filter(email => email) : undefined})}
          />
        </div>
        <div className="form-group">
          <label htmlFor="bcc">BCC (optional):</label>
          <input
            type="text"
            id="bcc"
            placeholder="BCC (comma-separated emails)"
            value={emailForm.bcc?.join(', ') || ''}
            onChange={(e) => setEmailForm({...emailForm, bcc: e.target.value ? e.target.value.split(',').map(email => email.trim()).filter(email => email) : undefined})}
          />
        </div>
        <div className="form-group">
          <label htmlFor="subject">Subject:</label>
          <input
            type="text"
            id="subject"
            placeholder="Subject"
            value={emailForm.subject}
            onChange={(e) => setEmailForm({...emailForm, subject: e.target.value})}
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="body">Email Body:</label>
          <textarea
            id="body"
            placeholder="Email Body"
            value={emailForm.body}
            onChange={(e) => setEmailForm({...emailForm, body: e.target.value})}
            rows={10}
            required
          />
        </div>
        <button type="submit" disabled={isLoading || emailAccounts.length === 0}>
          {isLoading ? 'Sending...' : 'Send Email'}
        </button>
        {emailAccounts.length === 0 && (
          <p className="warning">Please add an email account first.</p>
        )}
      </form>
    </div>
  );
};

export default ComposeEmailPage;