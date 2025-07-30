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
  DocumentationPage
} from './pages';
import { EmailTemplate } from './types';
import loadingAnimation from './assets/Figure Message sent.json';

function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [isNavigating, setIsNavigating] = useState(false);
  const [templateData, setTemplateData] = useState<{ subject: string; html_content: string; text_content: string } | null>(null);
  
  // Custom hooks
  const { user, isAuthenticated, logout } = useAuth();
  const {
    emailAccounts,
    emailTemplates,
    automationRules,
    stats,
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
  const handleUseTemplate = (template: { subject: string; html_content: string; text_content: string }) => {
    setTemplateData(template);
    handleTabChange('compose');
  };

  const handleCreateTemplate = async (templateData: Omit<EmailTemplate, 'id' | 'user_id' | 'created_at'>) => {
    // Convert EmailTemplate format to TemplateForm format expected by createTemplate
    const convertedData = {
      name: templateData.name,
      subject: templateData.subject,
      html_content: templateData.html_content,
      text_content: templateData.text_content || '',
      variables: templateData.variables
    };
    await createTemplate(convertedData);
  };

  // Show authentication if not logged in
  if (!isAuthenticated || !user) {
    return (
      <ThemeProvider>
        <div className="app">
          <AuthPage onAuthSuccess={loadData} />
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
      default: return 'Overview of your email automation';
    }
  };

  const renderPageContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return (
          <DashboardPage
            stats={stats}
            emailAccounts={emailAccounts}
            emailTemplates={emailTemplates}
            automationRules={automationRules}
          />
        );
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
      case 'documentation':
        return <DocumentationPage />;
      default:
        return (
          <DashboardPage
            stats={stats}
            emailAccounts={emailAccounts}
            emailTemplates={emailTemplates}
            automationRules={automationRules}
          />
        );
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
