import React, { useState } from 'react';
import '../styles/DocumentationPage.css';

const DocumentationPage: React.FC = () => {
  const [activeSection, setActiveSection] = useState('getting-started');

  const navigationItems = [
    { id: 'getting-started', label: 'Getting Started', icon: '🚀' },
    { id: 'quick-start', label: 'Quick Start', icon: '⚡' },
    { id: 'selectors', label: 'Selectors', icon: '🎯' },
    { id: 'data-types', label: 'Data Types', icon: '📊' },
    { id: 'scheduling', label: 'Scheduling', icon: '⏰' },
    { id: 'exporting', label: 'Exporting', icon: '📤' },
    { id: 'email-setup', label: 'Email Setup', icon: '📧' },
    { id: 'best-practices', label: 'Best Practices', icon: '✅' },
    { id: 'troubleshooting', label: 'Troubleshooting', icon: '🔧' },
    { id: 'ui-features', label: 'UI Features', icon: '🎨' },
  ];

  const renderContent = () => {
    switch (activeSection) {
      case 'getting-started':
        return (
          <div className="doc-content">
            <h1>Getting Started</h1>
            <p className="doc-subtitle">
              Email Automation Bot is compatible with almost every email provider. Select yours and get started!
            </p>
            
            <div className="feature-grid">
              <div className="feature-card">
                <div className="feature-icon">🎯</div>
                <h3>Multiple Email Providers</h3>
                <p>Gmail, Outlook, Yahoo, and custom SMTP servers for maximum flexibility</p>
              </div>
              
              <div className="feature-card">
                <div className="feature-icon">📊</div>
                <h3>Rich Templates</h3>
                <p>HTML templates, plain text, and dynamic content with variables</p>
              </div>
              
              <div className="feature-card">
                <div className="feature-icon">🔔</div>
                <h3>Smart Scheduling</h3>
                <p>Run campaigns at specific intervals, times, or trigger manually</p>
              </div>
              
              <div className="feature-card">
                <div className="feature-icon">📤</div>
                <h3>Export Options</h3>
                <p>CSV, JSON, and beautifully formatted HTML reports</p>
              </div>
            </div>
            
            <div className="what-is-section">
              <h2>What is Email Automation Bot?</h2>
              <p>
                Email Automation Bot is a powerful desktop application that allows you to automate email campaigns and manage 
                multiple email accounts efficiently. Built with modern technologies, it provides an intuitive interface for 
                creating, managing, and scheduling email campaigns without requiring technical expertise.
              </p>
              
              <div className="key-features">
                <h3>Key Features:</h3>
                <ul>
                  <li>Visual template builder with real-time preview</li>
                  <li>Support for multiple email providers and SMTP servers</li>
                  <li>Built-in analytics and tracking capabilities</li>
                  <li>Automatic personalization and variable substitution</li>
                  <li>Export campaign data to multiple formats</li>
                  <li>Scheduled execution with flexible timing options</li>
                  <li>Real-time notifications for campaign status</li>
                  <li>Advanced automation rules and triggers</li>
                </ul>
              </div>
            </div>
          </div>
        );
      
      case 'quick-start':
        return (
          <div className="doc-content">
            <h1>Quick Start</h1>
            <p className="doc-subtitle">
              Get up and running with Email Automation Bot in just a few minutes.
            </p>
            
            <div className="steps-container">
              <div className="step">
                <div className="step-number">1</div>
                <div className="step-content">
                  <h3>Add Email Account</h3>
                  <p>Connect your email provider by adding your account credentials. We support Gmail, Outlook, Yahoo, and custom SMTP servers.</p>
                  <div className="code-block">
                    <code>Navigate to Email Accounts → Add Account → Enter credentials</code>
                  </div>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">2</div>
                <div className="step-content">
                  <h3>Create Email Template</h3>
                  <p>Design your email template using our visual editor. Add variables, styling, and dynamic content.</p>
                  <div className="code-block">
                    <code>Templates → Create New → Design your email → Save template</code>
                  </div>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">3</div>
                <div className="step-content">
                  <h3>Set Up Automation</h3>
                  <p>Create automation rules to trigger emails based on conditions, schedules, or manual triggers.</p>
                  <div className="code-block">
                    <code>Automation → Create Rule → Set conditions → Link template</code>
                  </div>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">4</div>
                <div className="step-content">
                  <h3>Launch Campaign</h3>
                  <p>Review your setup and launch your email campaign. Monitor progress from the dashboard.</p>
                  <div className="code-block">
                    <code>Dashboard → Active Campaigns → Monitor results</code>
                  </div>
                </div>
              </div>
            </div>
          </div>
        );
      
      case 'selectors':
        return (
          <div className="doc-content">
            <h1>Email Selectors</h1>
            <p className="doc-subtitle">
              Learn how to target specific emails and create effective automation rules.
            </p>
            
            <div className="selector-types">
              <div className="selector-card">
                <h3>📧 Subject Line Selectors</h3>
                <p>Target emails based on subject line patterns:</p>
                <div className="code-block">
                  <code>subject:contains("Invoice")</code><br/>
                  <code>subject:startsWith("Order #")</code><br/>
                  <code>subject:regex("\\d{4}-\\d{2}-\\d{2}")</code>
                </div>
              </div>
              
              <div className="selector-card">
                <h3>👤 Sender Selectors</h3>
                <p>Filter emails by sender information:</p>
                <div className="code-block">
                  <code>from:"noreply@company.com"</code><br/>
                  <code>from:domain("amazon.com")</code><br/>
                  <code>from:contains("support")</code>
                </div>
              </div>
              
              <div className="selector-card">
                <h3>📅 Date Selectors</h3>
                <p>Target emails within specific time ranges:</p>
                <div className="code-block">
                  <code>date:after("2024-01-01")</code><br/>
                  <code>date:before("2024-12-31")</code><br/>
                  <code>date:last("7 days")</code>
                </div>
              </div>
              
              <div className="selector-card">
                <h3>📎 Content Selectors</h3>
                <p>Target emails based on content and attachments:</p>
                <div className="code-block">
                  <code>body:contains("tracking number")</code><br/>
                  <code>attachment:type("pdf")</code><br/>
                  <code>html:xpath("//table[@class='invoice']")</code>
                </div>
              </div>
            </div>
          </div>
        );
      
      case 'data-types':
        return (
          <div className="doc-content">
            <h1>Data Types</h1>
            <p className="doc-subtitle">
              Understanding the different types of data you can work with in email campaigns.
            </p>
            
            <div className="data-types-grid">
              <div className="data-type-card">
                <div className="data-type-icon">📝</div>
                <h3>Text Data</h3>
                <p>Work with various text formats in your email campaigns.</p>
                <ul>
                  <li>Plain text content</li>
                  <li>Rich text formatting</li>
                  <li>Dynamic variables</li>
                  <li>Personalization tokens</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">🔗</div>
                <h3>Links & URLs</h3>
                <p>Manage links and tracking in your email campaigns.</p>
                <ul>
                  <li>Click tracking</li>
                  <li>UTM parameters</li>
                  <li>Shortened URLs</li>
                  <li>Custom redirects</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">🖼️</div>
                <h3>Images</h3>
                <p>Handle images and media in your email templates.</p>
                <ul>
                  <li>Inline images</li>
                  <li>Image optimization</li>
                  <li>Alt text support</li>
                  <li>Responsive images</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">📊</div>
                <h3>Tables & Data</h3>
                <p>Structure data presentation in your emails.</p>
                <ul>
                  <li>HTML tables</li>
                  <li>Data formatting</li>
                  <li>Dynamic content</li>
                  <li>Conditional display</li>
                </ul>
              </div>
            </div>
          </div>
        );
      
      case 'scheduling':
        return (
          <div className="doc-content">
            <h1>Scheduling</h1>
            <p className="doc-subtitle">
              Learn how to schedule and automate your email campaigns effectively.
            </p>
            
            <h2>Scheduling Options</h2>
            <div className="selector-types">
              <div className="selector-card">
                <h3>Immediate Sending</h3>
                <p>Send emails immediately when triggered or manually launched.</p>
                <div className="code-block">
                  <code>trigger: "immediate", delay: 0</code>
                </div>
              </div>
              
              <div className="selector-card">
                <h3>Scheduled Campaigns</h3>
                <p>Schedule email campaigns for specific dates and times.</p>
                <div className="code-block">
                  <code>schedule: "2024-01-15 09:00", timezone: "UTC"</code>
                </div>
              </div>
              
              <div className="selector-card">
                <h3>Recurring Campaigns</h3>
                <p>Set up recurring email campaigns with flexible intervals.</p>
                <div className="code-block">
                  <code>interval: "daily", time: "09:00", weekdays_only: true</code>
                </div>
              </div>
              
              <div className="selector-card">
                <h3>Conditional Sending</h3>
                <p>Send emails based on specific conditions and triggers.</p>
                <div className="code-block">
                  <code>condition: "subscriber_count {'>'} 100", action: "send_campaign"</code>
                </div>
              </div>
            </div>
            
            <h2>Best Practices</h2>
            <ul>
              <li>Consider email provider rate limits when scheduling frequent campaigns</li>
              <li>Use off-peak hours for large batch sending</li>
              <li>Set up monitoring and alerts for failed campaigns</li>
              <li>Test scheduling with small recipient lists first</li>
              <li>Configure retry mechanisms for failed delivery attempts</li>
            </ul>
          </div>
        );
      
      case 'exporting':
        return (
          <div className="doc-content">
            <h1>Exporting</h1>
            <p className="doc-subtitle">
              Export your campaign data and analytics in various formats for analysis.
            </p>
            
            <h2>Export Formats</h2>
            <div className="data-types-grid">
              <div className="data-type-card">
                <div className="data-type-icon">📊</div>
                <h3>CSV Export</h3>
                <p>Export campaign data in CSV format for spreadsheet analysis.</p>
                <ul>
                  <li>Customizable column headers</li>
                  <li>Date range filtering</li>
                  <li>Encoding options (UTF-8, ASCII)</li>
                  <li>Delimiter customization</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">📄</div>
                <h3>JSON Export</h3>
                <p>Export structured data in JSON format for API integration.</p>
                <ul>
                  <li>Nested data structures</li>
                  <li>Metadata inclusion</li>
                  <li>Pretty-printed formatting</li>
                  <li>Schema validation</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">📋</div>
                <h3>HTML Reports</h3>
                <p>Generate beautiful HTML reports with charts and visualizations.</p>
                <ul>
                  <li>Interactive data tables</li>
                  <li>Charts and graphs</li>
                  <li>Custom styling and branding</li>
                  <li>Print-friendly layouts</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">📧</div>
                <h3>Email Reports</h3>
                <p>Automatically send campaign reports via email to stakeholders.</p>
                <ul>
                  <li>Scheduled report delivery</li>
                  <li>Multiple recipient support</li>
                  <li>Attachment options</li>
                  <li>Summary and detailed views</li>
                </ul>
              </div>
            </div>
          </div>
        );
      
      case 'email-setup':
        return (
          <div className="doc-content">
            <h1>Email Setup</h1>
            <p className="doc-subtitle">
              Configure your email accounts and SMTP settings for reliable delivery.
            </p>
            
            <h2>Supported Email Providers</h2>
            <div className="feature-grid">
              <div className="feature-card">
                <div className="feature-icon">📧</div>
                <h3>Gmail</h3>
                <p>Use OAuth2 or app passwords for secure Gmail integration</p>
              </div>
              
              <div className="feature-card">
                <div className="feature-icon">📨</div>
                <h3>Outlook</h3>
                <p>Connect with Microsoft 365 or Outlook.com accounts</p>
              </div>
              
              <div className="feature-card">
                <div className="feature-icon">📮</div>
                <h3>Yahoo Mail</h3>
                <p>Configure Yahoo Mail with app-specific passwords</p>
              </div>
              
              <div className="feature-card">
                <div className="feature-icon">⚙️</div>
                <h3>Custom SMTP</h3>
                <p>Use any SMTP server with custom configuration</p>
              </div>
            </div>
            
            <h2>Configuration Steps</h2>
            <div className="steps-container">
              <div className="step">
                <div className="step-number">1</div>
                <div className="step-content">
                  <h3>Add Email Account</h3>
                  <p>Navigate to Email Accounts and click "Add Account" to begin setup.</p>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">2</div>
                <div className="step-content">
                  <h3>Choose Provider</h3>
                  <p>Select your email provider or choose "Custom SMTP" for other services.</p>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">3</div>
                <div className="step-content">
                  <h3>Enter Credentials</h3>
                  <p>Provide your email address, password, and any required authentication.</p>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">4</div>
                <div className="step-content">
                  <h3>Test Connection</h3>
                  <p>Verify your settings by testing the connection before saving.</p>
                </div>
              </div>
            </div>
          </div>
        );
      
      case 'best-practices':
        return (
          <div className="doc-content">
            <h1>Best Practices</h1>
            <p className="doc-subtitle">
              Follow these guidelines to maximize the effectiveness of your email automation.
            </p>
            
            <h2>Email Campaign Best Practices</h2>
            <div className="steps-container">
              <div className="step">
                <div className="step-number">1</div>
                <div className="step-content">
                  <h3>Email Account Security</h3>
                  <p>Secure your email accounts and use proper authentication methods.</p>
                  <ul>
                    <li>Use OAuth2 authentication when available</li>
                    <li>Enable two-factor authentication</li>
                    <li>Use app-specific passwords for SMTP</li>
                    <li>Regularly rotate credentials</li>
                  </ul>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">2</div>
                <div className="step-content">
                  <h3>Content Optimization</h3>
                  <p>Create engaging and effective email content.</p>
                  <ul>
                    <li>Use clear and compelling subject lines</li>
                    <li>Personalize content with recipient data</li>
                    <li>Optimize for mobile devices</li>
                    <li>Include clear call-to-action buttons</li>
                  </ul>
                </div>
              </div>
              
              <div className="step">
                <div className="step-number">3</div>
                <div className="step-content">
                  <h3>Delivery Optimization</h3>
                  <p>Ensure high deliverability and avoid spam filters.</p>
                  <ul>
                    <li>Maintain clean recipient lists</li>
                    <li>Monitor bounce rates and unsubscribes</li>
                    <li>Use proper sender authentication</li>
                    <li>Respect sending frequency limits</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        );
      
      case 'troubleshooting':
        return (
          <div className="doc-content">
            <h1>Troubleshooting</h1>
            <p className="doc-subtitle">
              Common issues and solutions for Email Automation Bot.
            </p>
            
            <h2>Common Issues</h2>
            <div className="selector-types">
              <div className="selector-card">
                <h3>Authentication Errors</h3>
                <p>Issues with email account login or connection problems.</p>
                <ul>
                  <li>Verify username and password are correct</li>
                  <li>Check if two-factor authentication is enabled</li>
                  <li>Use app-specific passwords when required</li>
                  <li>Verify SMTP server settings</li>
                </ul>
              </div>
              
              <div className="selector-card">
                <h3>Delivery Issues</h3>
                <p>Problems with email delivery or high bounce rates.</p>
                <ul>
                  <li>Check recipient email addresses for validity</li>
                  <li>Monitor sender reputation and blacklists</li>
                  <li>Verify SPF, DKIM, and DMARC records</li>
                  <li>Review email content for spam triggers</li>
                </ul>
              </div>
              
              <div className="selector-card">
                <h3>Performance Problems</h3>
                <p>Slow sending speeds or application performance issues.</p>
                <ul>
                  <li>Check system resources and memory usage</li>
                  <li>Optimize email template complexity</li>
                  <li>Reduce batch sizes for large campaigns</li>
                  <li>Monitor network connectivity and latency</li>
                </ul>
              </div>
              
              <div className="selector-card">
                <h3>Template Issues</h3>
                <p>Problems with email template rendering or formatting.</p>
                <ul>
                  <li>Test templates across different email clients</li>
                  <li>Validate HTML markup and CSS</li>
                  <li>Check image links and attachments</li>
                  <li>Verify variable substitution works correctly</li>
                </ul>
              </div>
            </div>
          </div>
        );
      
      case 'ui-features':
        return (
          <div className="doc-content">
            <h1>UI Features</h1>
            <p className="doc-subtitle">
              Explore the user interface features and navigation options in Email Automation Bot.
            </p>
            
            <h2>Main Navigation</h2>
            <div className="data-types-grid">
              <div className="data-type-card">
                <div className="data-type-icon">📊</div>
                <h3>Dashboard</h3>
                <p>Overview of your email campaigns, statistics, and recent activity.</p>
                <ul>
                  <li>Campaign performance metrics</li>
                  <li>Recent sending activity</li>
                  <li>Quick action buttons</li>
                  <li>System status indicators</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">📧</div>
                <h3>Email Accounts</h3>
                <p>Manage your connected email accounts and server configurations.</p>
                <ul>
                  <li>Add and remove email accounts</li>
                  <li>Test connection status</li>
                  <li>Configure SMTP settings</li>
                  <li>Monitor account health</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">📝</div>
                <h3>Templates</h3>
                <p>Create and manage email templates and campaign content.</p>
                <ul>
                  <li>Visual template editor</li>
                  <li>Variable insertion and testing</li>
                  <li>Template preview and validation</li>
                  <li>Import/export templates</li>
                </ul>
              </div>
              
              <div className="data-type-card">
                <div className="data-type-icon">⚡</div>
                <h3>Automation</h3>
                <p>Set up and manage automation rules and scheduling for campaigns.</p>
                <ul>
                  <li>Create automation workflows</li>
                  <li>Set triggers and conditions</li>
                  <li>Schedule recurring campaigns</li>
                  <li>Monitor automation performance</li>
                </ul>
              </div>
            </div>
            
            <h2>Interface Features</h2>
            <ul>
              <li><strong>Dark/Light Theme:</strong> Toggle between dark and light themes for comfortable viewing</li>
              <li><strong>Responsive Design:</strong> Interface adapts to different screen sizes and resolutions</li>
              <li><strong>Real-time Updates:</strong> Live updates for campaign status and delivery metrics</li>
              <li><strong>Keyboard Shortcuts:</strong> Quick access to common actions and navigation</li>
              <li><strong>Contextual Help:</strong> Tooltips and help text throughout the interface</li>
              <li><strong>Notification System:</strong> In-app notifications for important events and updates</li>
            </ul>
          </div>
        );
       
       default:
         return (
           <div className="doc-content">
             <h1>Documentation</h1>
             <p>Select a topic from the sidebar to get started.</p>
           </div>
         );
    }
  };

  return (
    <div className="documentation-page">
      <div className="doc-sidebar">
        <div className="doc-sidebar-header">
          <div className="doc-logo">
            <span className="doc-logo-icon">📚</span>
            <span className="doc-logo-text">Documentation</span>
          </div>
        </div>
        
        <nav className="doc-nav">
          {navigationItems.map((item) => (
            <button
              key={item.id}
              className={`doc-nav-item ${activeSection === item.id ? 'active' : ''}`}
              onClick={() => setActiveSection(item.id)}
            >
              <span className="doc-nav-icon">{item.icon}</span>
              <span className="doc-nav-label">{item.label}</span>
            </button>
          ))}
        </nav>
      </div>
      
      <div className="doc-main">
        <div className="doc-content-wrapper">
          {renderContent()}
        </div>
      </div>
    </div>
  );
};

export default DocumentationPage;