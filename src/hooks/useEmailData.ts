import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { 
  EmailAccount, 
  EmailTemplate, 
  AutomationRule, 
  EmailStats, 
  DashboardStats,
  EmailAccountForm,
  TemplateForm,
  EmailForm
} from '../types';

export const useEmailData = () => {
  const [emailAccounts, setEmailAccounts] = useState<EmailAccount[]>([]);
  const [emailTemplates, setEmailTemplates] = useState<EmailTemplate[]>([]);
  const [automationRules, setAutomationRules] = useState<AutomationRule[]>([]);
  const [stats, setStats] = useState<DashboardStats>({
    total_emails: 0,
    sent_today: 0,
    active_rules: 0,
    success_rate: 0
  });
  const [isLoading, setIsLoading] = useState(false);

  const loadData = async (): Promise<void> => {
    const token = localStorage.getItem('auth_token');
    if (!token) return;
    
    try {
      const [accounts, templates, rules, statsData] = await Promise.all([
        invoke<EmailAccount[]>('get_email_accounts', { token }),
        invoke<EmailTemplate[]>('get_email_templates', { token }),
        invoke<AutomationRule[]>('get_automation_rules', { token }),
        invoke<EmailStats>('get_email_stats', { token })
      ]);
      
      setEmailAccounts(accounts);
      setEmailTemplates(templates);
      setAutomationRules(rules);
      setStats({
        total_emails: statsData.total_sent + statsData.total_received,
        sent_today: statsData.total_sent,
        active_rules: rules.filter(rule => rule.is_active).length,
        success_rate: statsData.success_rate * 100
      });
    } catch (error) {
      throw error;
    }
  };

  const createEmailAccount = async (accountData: EmailAccountForm): Promise<void> => {
    const token = localStorage.getItem('auth_token');
    if (!token) return;
    
    setIsLoading(true);
    try {
      await invoke('create_email_account', { token, accountData });
      await loadData();
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const testConnection = async (accountId: number): Promise<any> => {
    const token = localStorage.getItem('auth_token');
    if (!token) return;
    
    setIsLoading(true);
    try {
      const result = await invoke('test_email_connection', { token, accountId });
      return result;
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const createTemplate = async (templateData: Omit<TemplateForm, 'variables'> & { variables: string[] }): Promise<void> => {
    const token = localStorage.getItem('auth_token');
    if (!token) return;
    
    setIsLoading(true);
    try {
      await invoke('create_email_template', { token, templateData });
      await loadData();
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const sendEmail = async (emailData: Omit<EmailForm, 'to'> & { to: string[] }): Promise<void> => {
    const token = localStorage.getItem('auth_token');
    if (!token || emailAccounts.length === 0) return;
    
    setIsLoading(true);
    try {
      const activeAccount = emailAccounts.find(acc => acc.is_active);
      if (!activeAccount) {
        throw new Error('No active email account found');
      }
      
      // Convert EmailForm to EmailMessage format expected by backend
      const emailMessage = {
        to: emailData.to,
        cc: [],
        bcc: [],
        subject: emailData.subject,
        body: emailData.html_body || emailData.text_body,
        attachments: []
      };
      
      await invoke('send_email', { 
        token, 
        account_id: activeAccount.id, 
        email_data: emailMessage 
      });
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  return {
    emailAccounts,
    emailTemplates,
    automationRules,
    stats,
    isLoading,
    loadData,
    createEmailAccount,
    testConnection,
    createTemplate,
    sendEmail
  };
};