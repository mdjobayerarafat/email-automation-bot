import { useState, useEffect } from 'react';
import './App.css';
import Lottie from 'lottie-react';
import { ThemeProvider } from './contexts/ThemeContext';
import Sidebar from './components/Sidebar';
import Header from './components/Header';
import NotificationBar from './components/NotificationBar';
import { useAuth, useEmailData, useNotifications } from './hooks';
import {
  AuthPage,
  DashboardPage,
  EmailAccountsPage,
  EmailTemplatesPage,
  AutomationRulesPage,
  ComposeEmailPage,
  DocumentationPage,
  InboxMonitorPage,
  ContactListsPage,
  CampaignsPage,
  AttachmentsPage
} from './pages';
import { EmailTemplate } from './types';
import loadingAnimation from './assets/Figure Message sent.json';

function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [isNavigating, setIsNavigating] = useState(false);
  const [templateData, setTemplateData] = useState<{ subject: string; body: string } | null>(null);
  
  // Custom hooks
  const { user, isAuthenticated, token, logout } = useAuth();
  const {
    emailAccounts,
    emailTemplates,
    automationRules,
    isLoading: dataLoading,
    loadData,
    createEmailAccount,
    testConnection,
    createTemplate,
    sendEmail
  } = useEmailData();
  const { error, success, showError, showSuccess } = useNotifications();

  // Load data when authenticated
  useEffect(() => {
    if (isAuthenticated) {
      loadData();
    }
  }, [isAuthenticated, loadData]);
  
  // Handle navigation with loading animation
  const handleTabChange = (newTab: string) => {
    if (newTab === activeTab) return;
    
    setIsNavigating(true);
    setTimeout(() => {
      setActiveTab(newTab);
      setTimeout(() => {
        setIsNavigating(false);
      }, 800); // Animation duration
    }, 200);
  };

  // Handle template usage
  const handleUseTemplate = (template: { subject?: string; body?: string }) => {
    setTemplateData(template.subject && template.body ? { subject: template.subject, body: template.body } : null);
    handleTabChange('compose');
  };

  const handleCreateTemplate = async (templateData: Omit<EmailTemplate, 'id' | 'user_id' | 'created_at' | 'updated_at'>) => {
    // Convert EmailTemplate format to TemplateForm format expected by createTemplate
    const convertedData = {
      name: templateData.name,
      subject: templateData.subject,
      body: templateData.body,
      template_type: templateData.template_type,
      variables: []
    };
    await createTemplate(convertedData);
  };

  // Show authentication if not logged in
  if (!isAuthenticated || !user) {
    return (
      <ThemeProvider>
        <div className="app">
          <AuthPage onAuthSuccess={() => {}} />
        </div>
      </ThemeProvider>
    );
  }

  const getPageTitle = () => {
    switch (activeTab) {
      case 'dashboard': return 'Dashboard';
      case 'accounts': return 'Email Accounts';
      case 'templates': return 'Email Templates';
      case 'automation': return 'Automation Rules';
      case 'compose': return 'Compose Email';
      case 'inbox-monitor': return 'Inbox Monitor';
      case 'contact-lists': return 'Contact Lists';
      case 'campaigns': return 'Campaigns';
      case 'attachments': return 'Attachments';
      case 'documentation': return 'Documentation';
      default: return 'Dashboard';
    }
  };

  const getPageSubtitle = () => {
    switch (activeTab) {
      case 'dashboard': return 'Overview of your email automation';
      case 'accounts': return 'Manage your email accounts';
      case 'templates': return 'Create and manage email templates';
      case 'automation': return 'Set up automation rules';
      case 'compose': return 'Send emails manually';
      case 'inbox-monitor': return 'Monitor inbox for keywords and automate responses';
      case 'contact-lists': return 'Manage contact lists and import contacts';
      case 'campaigns': return 'Create and manage bulk email campaigns';
      case 'attachments': return 'View and manage email attachments';
      case 'documentation': return 'Learn how to use the application';
      default: return 'Overview of your email automation';
    }
  };

  const renderPageContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <DashboardPage />;
      case 'accounts':
        return (
          <EmailAccountsPage
            emailAccounts={emailAccounts}
            isLoading={dataLoading}
            onCreateAccount={createEmailAccount}
            onTestConnection={testConnection}
            onError={showError}
            onSuccess={showSuccess}
          />
        );
      case 'templates':
        return (
          <EmailTemplatesPage
            emailTemplates={emailTemplates}
            isLoading={dataLoading}
            onCreateTemplate={handleCreateTemplate}
            onError={showError}
            onSuccess={showSuccess}
            onUseTemplate={handleUseTemplate}
          />
        );
      case 'automation':
        return <AutomationRulesPage automationRules={automationRules} />;
      case 'compose':
        return (
          <ComposeEmailPage
            emailAccounts={emailAccounts}
            isLoading={dataLoading}
            onSendEmail={sendEmail}
            onError={showError}
            onSuccess={showSuccess}
            templateData={templateData}
            onClearTemplate={() => setTemplateData(null)}
          />
        );
      case 'inbox-monitor':
        return <InboxMonitorPage token={token || ''} />;
      case 'contact-lists':
        return <ContactListsPage token={token || ''} />;
      case 'campaigns':
        return <CampaignsPage token={token || ''} />;
      case 'attachments':
        return <AttachmentsPage token={token || ''} />;
      case 'documentation':
        return <DocumentationPage />;
      default:
        return <DashboardPage />;
    }
  };

  return (
    <ThemeProvider>
      <div className="app">
        <Sidebar 
          activeTab={activeTab} 
          setActiveTab={handleTabChange} 
          currentUser={user}
          onLogout={logout}
        />
        
        <div className="main-content">
          <Header 
            title={getPageTitle()}
            subtitle={getPageSubtitle()}
          />
          
          <div className="content-area">
            <NotificationBar error={error} success={success} />
            {!isNavigating && renderPageContent()}
          </div>
        </div>
        
        {/* Loading overlay during navigation */}
        {isNavigating && (
          <div className="navigation-loading-overlay">
            <div className="loading-content">
              <Lottie 
                animationData={loadingAnimation} 
                style={{ width: 200, height: 200 }}
                loop={true}
              />
              <p className="loading-text">Loading...</p>
            </div>
          </div>
        )}
      </div>
    </ThemeProvider>
  );
}

export default App;
