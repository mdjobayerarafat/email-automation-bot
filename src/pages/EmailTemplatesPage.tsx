import React, { useState } from 'react';
import { EmailTemplate, TemplateForm } from '../types';
import { 
  getPremadeTemplates, 
  getTemplatesByCategory, 
  getAllCategories, 
  PremadeTemplate 
} from '../services/templateService';
import '../styles/EmailTemplatesPage.css';

interface EmailTemplatesPageProps {
  emailTemplates: EmailTemplate[];
  isLoading: boolean;
  onCreateTemplate: (templateData: Omit<EmailTemplate, 'id' | 'user_id' | 'created_at' | 'updated_at'>) => Promise<void>;
  onError: (message: string) => void;
  onSuccess: (message: string) => void;
  onUseTemplate?: (template: { subject?: string; body?: string }) => void;
}

const EmailTemplatesPage: React.FC<EmailTemplatesPageProps> = ({
  emailTemplates: propEmailTemplates,
  onCreateTemplate,
  onError,
  onSuccess,
  onUseTemplate
}) => {
  const [premadeTemplates] = useState<PremadeTemplate[]>(getPremadeTemplates());
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [activeTab, setActiveTab] = useState<'premade' | 'custom'>('premade');
  const [previewTemplate, setPreviewTemplate] = useState<PremadeTemplate | null>(null);
  const [showPreview, setShowPreview] = useState(false);
  const [templateForm, setTemplateForm] = useState<TemplateForm>({
    name: '',
    subject: '',
    body: '',
    template_type: 'email'
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await onCreateTemplate({
        name: templateForm.name,
        subject: templateForm.subject,
        body: templateForm.body,
        template_type: templateForm.template_type
      });

      // Reset form
      setTemplateForm({
        name: '',
        subject: '',
        body: '',
        template_type: 'email'
      });

      onSuccess('Template created successfully!');
    } catch (error) {
      console.error('Error creating template:', error);
      onError('Failed to create template');
    }
  };

  const handleUsePremadeTemplate = (template: PremadeTemplate) => {
    setTemplateForm({
      name: template.name,
      subject: template.subject,
      body: template.html_content,
      template_type: 'email'
    });
    setActiveTab('custom');
  };

  const handlePreviewTemplate = (template: PremadeTemplate) => {
    setPreviewTemplate(template);
    setShowPreview(true);
  };

  const closePreview = () => {
    setShowPreview(false);
    setPreviewTemplate(null);
  };

  const filteredPremadeTemplates = selectedCategory === 'all' 
    ? premadeTemplates 
    : getTemplatesByCategory(selectedCategory);

  // Categories are handled directly in the select options

  return (
    <div className="email-templates-page">
      <h2>Email Templates</h2>
      
      {/* Tab Navigation */}
      <div className="tabs">
        <button 
          className={`tab ${activeTab === 'premade' ? 'active' : ''}`}
          onClick={() => setActiveTab('premade')}
        >
          Pre-made Templates
        </button>
        <button 
          className={`tab ${activeTab === 'custom' ? 'active' : ''}`}
          onClick={() => setActiveTab('custom')}
        >
          Custom Templates
        </button>
      </div>

      {/* Pre-made Templates Tab */}
      {activeTab === 'premade' && (
        <div>
          <h3>Pre-made Templates</h3>
          
          {/* Category Filter */}
          <div className="category-filter">
            <label>Filter by Category:</label>
            <select 
              value={selectedCategory} 
              onChange={(e) => setSelectedCategory(e.target.value)}
            >
              <option value="all">All Categories</option>
              {getAllCategories().map(category => (
                <option key={category} value={category}>{category}</option>
              ))}
            </select>
          </div>
          
          {/* Templates Grid */}
          <div className="templates-grid">
            {filteredPremadeTemplates.map(template => (
              <div key={template.id} className="template-card">
                <h4>{template.name}</h4>
                <span className="category">{template.category}</span>
                <p><strong>Subject:</strong> {template.subject}</p>
                <p>{template.description}</p>
                <div className="template-actions">
                  <button 
                    className="preview-btn"
                    onClick={() => handlePreviewTemplate(template)}
                  >
                    Preview
                  </button>
                  <button 
                    className="use-template-btn"
                    onClick={() => handleUsePremadeTemplate(template)}
                  >
                    Use This Template
                  </button>
                  {onUseTemplate && (
                    <button 
                      className="use-now-btn"
                      onClick={() => onUseTemplate({
                        subject: template.subject,
                        body: template.html_content
                      })}
                    >
                      Use Now
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'custom' && (
        <div className="custom-templates">
          {/* Template Form */}
          <div className="form">
            <h3>Create Custom Template</h3>
            <form onSubmit={handleSubmit}>
              <input
                type="text"
                placeholder="Template Name"
                value={templateForm.name}
                onChange={(e) => setTemplateForm({...templateForm, name: e.target.value})}
                required
              />
              
              <input
                type="text"
                placeholder="Subject (optional)"
                value={templateForm.subject || ''}
                onChange={(e) => setTemplateForm({...templateForm, subject: e.target.value || undefined})}
              />
              
              <textarea
                placeholder="Template Body (optional)"
                value={templateForm.body || ''}
                onChange={(e) => setTemplateForm({...templateForm, body: e.target.value || undefined})}
                rows={10}
              />
              
              <select
                value={templateForm.template_type || 'email'}
                onChange={(e) => setTemplateForm({...templateForm, template_type: e.target.value || undefined})}
              >
                <option value="email">Email Template</option>
                <option value="sms">SMS Template</option>
                <option value="notification">Notification Template</option>
              </select>
              
              <button type="submit">
                Create Template
              </button>
            </form>
          </div>
          
          {/* Existing Templates */}
          <div className="templates-list">
            <h3>Your Custom Templates</h3>
            {propEmailTemplates.length === 0 ? (
              <div className="empty-state">
                <h3>No templates created yet</h3>
                <p>Create your first custom email template using the form above.</p>
              </div>
            ) : (
              propEmailTemplates.map(template => (
                <div key={template.id} className="template-card">
                  <h4>{template.name}</h4>
                  {template.subject && <p><strong>Subject:</strong> {template.subject}</p>}
                  {template.template_type && <p><strong>Type:</strong> {template.template_type}</p>}
                  <div className="template-content">
                    {template.body && (
                      <details>
                        <summary>View Template Body</summary>
                        <div>
                          <pre>{template.body}</pre>
                        </div>
                      </details>
                    )}
                  </div>
                  {onUseTemplate && (
                    <div className="template-actions">
                      <button 
                        className="use-now-btn"
                        onClick={() => onUseTemplate({
                          subject: template.subject,
                          body: template.body
                        })}
                      >
                        Use Now
                      </button>
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        </div>
      )}

      {/* Preview Modal */}
      {showPreview && previewTemplate && (
        <div className="preview-modal">
          <div className="preview-modal-content">
            <div className="preview-header">
              <h3>Preview: {previewTemplate.name}</h3>
              <button className="close-btn" onClick={closePreview}>&times;</button>
            </div>
            <div className="preview-body">
              <div className="preview-info">
                <p><strong>Subject:</strong> {previewTemplate.subject}</p>
                <p><strong>Category:</strong> {previewTemplate.category}</p>
                {previewTemplate.variables.length > 0 && (
                  <p><strong>Variables:</strong> {previewTemplate.variables.join(', ')}</p>
                )}
              </div>
              <div className="preview-tabs">
                <div className="preview-content">
                  <h4>HTML Preview</h4>
                  <div className="html-preview">
                    <iframe 
                      srcDoc={previewTemplate.html_content}
                      title="Email Preview"
                      style={{width: '100%', height: '400px', border: '1px solid #ddd', borderRadius: '4px'}}
                    />
                  </div>
                  {previewTemplate.text_content && (
                    <div className="text-preview">
                      <h4>Text Preview</h4>
                      <pre style={{whiteSpace: 'pre-wrap', background: '#f5f5f5', padding: '15px', borderRadius: '4px'}}>
                        {previewTemplate.text_content}
                      </pre>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default EmailTemplatesPage;