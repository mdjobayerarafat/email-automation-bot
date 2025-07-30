import React from 'react';
import { DashboardStats, EmailAccount, EmailTemplate, AutomationRule } from '../types';

interface DashboardPageProps {
  stats: DashboardStats;
  emailAccounts: EmailAccount[];
  emailTemplates: EmailTemplate[];
  automationRules: AutomationRule[];
}

const DashboardPage: React.FC<DashboardPageProps> = ({
  stats,
  emailAccounts,
  emailTemplates,
  automationRules
}) => {
  return (
    <div className="dashboard">
      <div className="stats-grid">
        <div className="stat-card">
          <h3>Total Emails</h3>
          <p>{stats.total_emails}</p>
        </div>
        <div className="stat-card">
          <h3>Sent Today</h3>
          <p>{stats.sent_today}</p>
        </div>
        <div className="stat-card">
          <h3>Active Rules</h3>
          <p>{stats.active_rules}</p>
        </div>
        <div className="stat-card">
          <h3>Success Rate</h3>
          <p>{stats.success_rate.toFixed(1)}%</p>
        </div>
      </div>
      
      <div className="quick-stats">
        <div className="stat-item">
          <span>Email Accounts:</span>
          <span>{emailAccounts.length}</span>
        </div>
        <div className="stat-item">
          <span>Templates:</span>
          <span>{emailTemplates.length}</span>
        </div>
        <div className="stat-item">
          <span>Automation Rules:</span>
          <span>{automationRules.length}</span>
        </div>
      </div>
    </div>
  );
};

export default DashboardPage;