import React, { useState } from 'react';
import { EmailAccount, EmailAccountForm } from '../types';
import '../styles/EmailAccountsPage.css';

interface EmailAccountsPageProps {
  emailAccounts: EmailAccount[];
  isLoading: boolean;
  onCreateAccount: (accountData: EmailAccountForm) => Promise<void>;
  onTestConnection: (accountId: number) => Promise<void>;
  onError: (message: string) => void;
  onSuccess: (message: string) => void;
}

const EmailAccountsPage: React.FC<EmailAccountsPageProps> = ({
  emailAccounts,
  isLoading,
  onCreateAccount,
  onTestConnection,
  onError,
  onSuccess
}) => {
  const [emailAccountForm, setEmailAccountForm] = useState<EmailAccountForm>({
    account_name: '',
    email_address: '',
    username: '',
    password: '',
    smtp_server: '',
    smtp_port: 587,
    imap_server: '',
    imap_port: 993
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await onCreateAccount(emailAccountForm);
      onSuccess('Email account created successfully!');
      setEmailAccountForm({
        account_name: '',
        email_address: '',
        username: '',
        password: '',
        smtp_server: '',
        smtp_port: 587,
        imap_server: '',
        imap_port: 993
      });
    } catch (err) {
      onError(err as string);
    }
  };

  const handleTestConnection = async (accountId: number) => {
    try {
      const result = await onTestConnection(accountId);
      onSuccess('Connection test completed. Check console for details.');
      console.log('Connection test result:', result);
    } catch (err) {
      onError(err as string);
    }
  };

  return (
    <div className="accounts">
      <form onSubmit={handleSubmit} className="form">
        <h3>Add New Email Account</h3>
        <input
          type="text"
          placeholder="Account Name (e.g., Work Gmail, Personal Outlook)"
          value={emailAccountForm.account_name}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, account_name: e.target.value})}
          required
        />
        <input
          type="email"
          placeholder="Email Address"
          value={emailAccountForm.email_address}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, email_address: e.target.value})}
          required
        />
        <input
          type="text"
          placeholder="Username (for SMTP/IMAP login)"
          value={emailAccountForm.username}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, username: e.target.value})}
          required
        />
        <input
          type="password"
          placeholder="Password"
          value={emailAccountForm.password}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, password: e.target.value})}
          required
        />
        <input
          type="text"
          placeholder="SMTP Server (optional)"
          value={emailAccountForm.smtp_server || ''}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, smtp_server: e.target.value || undefined})}
        />
        <input
          type="number"
          placeholder="SMTP Port (optional)"
          value={emailAccountForm.smtp_port || ''}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, smtp_port: e.target.value ? parseInt(e.target.value) : undefined})}
        />
        <input
          type="text"
          placeholder="IMAP Server (optional)"
          value={emailAccountForm.imap_server || ''}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, imap_server: e.target.value || undefined})}
        />
        <input
          type="number"
          placeholder="IMAP Port (optional)"
          value={emailAccountForm.imap_port || ''}
          onChange={(e) => setEmailAccountForm({...emailAccountForm, imap_port: e.target.value ? parseInt(e.target.value) : undefined})}
        />
        <button type="submit" disabled={isLoading}>
          {isLoading ? 'Adding...' : 'Add Account'}
        </button>
      </form>

      <div className="accounts-list">
        <h3>Your Email Accounts</h3>
        {emailAccounts.map(account => (
          <div key={account.id} className="account-card">
            <div className="account-info">
              <h4>{account.account_name}</h4>
              <p className="email-address">{account.email_address}</p>
              {account.smtp_server && account.smtp_port && (
                <p>SMTP: {account.smtp_server}:{account.smtp_port}</p>
              )}
              {account.imap_server && account.imap_port && (
                <p>IMAP: {account.imap_server}:{account.imap_port}</p>
              )}
              <p>Status: {account.is_active ? 'Active' : 'Inactive'}</p>
            </div>
            <button onClick={() => handleTestConnection(account.id)}>
              Test Connection
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default EmailAccountsPage;