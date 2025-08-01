import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../styles/DashboardPage.css';

interface DashboardStats {
  total_emails_sent: number;
  total_emails_failed: number;
  total_contacts: number;
  total_templates: number;
  total_campaigns: number;
  active_monitors: number;
}

interface RecentActivity {
  id: number;
  activity_type: string;
  description: string;
  timestamp: string;
}

const DashboardPage: React.FC = () => {
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [recentActivity, setRecentActivity] = useState<RecentActivity[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const dashboardStats = await invoke<DashboardStats>('get_dashboard_stats');
      setStats(dashboardStats);
      
      // Load recent activity (last 10 email logs)
      const logs = await invoke<any[]>('get_email_logs', { limit: 10 });
      const activity: RecentActivity[] = logs.map((log, index) => ({
        id: index,
        activity_type: log.status === 'sent' ? 'email_sent' : 'email_failed',
        description: `Email to ${log.recipient}: ${log.subject}`,
        timestamp: log.sent_at
      }));
      setRecentActivity(activity);
    } catch (err) {
      console.error('Failed to load dashboard data:', err);
      setError('Failed to load dashboard data');
    } finally {
      setLoading(false);
    }
  };

  const getActivityIcon = (type: string) => {
    switch (type) {
      case 'email_sent':
        return '✅';
      case 'email_failed':
        return '❌';
      case 'campaign_created':
        return '📧';
      case 'monitor_triggered':
        return '🔔';
      default:
        return '📝';
    }
  };

  const formatTimestamp = (timestamp: string) => {
    return new Date(timestamp).toLocaleString();
  };

  const calculateSuccessRate = () => {
    if (!stats) return 0;
    const total = stats.total_emails_sent + stats.total_emails_failed;
    if (total === 0) return 0;
    return Math.round((stats.total_emails_sent / total) * 100);
  };

  if (loading) {
    return (
      <div className="dashboard-page">
        <div className="loading">Loading dashboard...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="dashboard-page">
        <div className="error-message">{error}</div>
        <button onClick={loadDashboardData} className="btn btn-primary">
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="dashboard-page">
      <div className="page-header">
        <h1>Dashboard</h1>
        <button onClick={loadDashboardData} className="btn btn-outline">
          🔄 Refresh
        </button>
      </div>

      {/* Stats Overview */}
      <div className="stats-grid">
        <div className="stat-card emails">
          <div className="stat-icon">📧</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.total_emails_sent || 0}</div>
            <div className="stat-label">Emails Sent</div>
            <div className="stat-subtitle">
              {stats?.total_emails_failed || 0} failed
            </div>
          </div>
        </div>

        <div className="stat-card success-rate">
          <div className="stat-icon">📊</div>
          <div className="stat-content">
            <div className="stat-value">{calculateSuccessRate()}%</div>
            <div className="stat-label">Success Rate</div>
            <div className="stat-subtitle">
              {(stats?.total_emails_sent || 0) + (stats?.total_emails_failed || 0)} total
            </div>
          </div>
        </div>

        <div className="stat-card contacts">
          <div className="stat-icon">👥</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.total_contacts || 0}</div>
            <div className="stat-label">Contacts</div>
            <div className="stat-subtitle">across all lists</div>
          </div>
        </div>

        <div className="stat-card templates">
          <div className="stat-icon">📝</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.total_templates || 0}</div>
            <div className="stat-label">Templates</div>
            <div className="stat-subtitle">ready to use</div>
          </div>
        </div>

        <div className="stat-card campaigns">
          <div className="stat-icon">🚀</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.total_campaigns || 0}</div>
            <div className="stat-label">Campaigns</div>
            <div className="stat-subtitle">created</div>
          </div>
        </div>

        <div className="stat-card monitors">
          <div className="stat-icon">🔔</div>
          <div className="stat-content">
            <div className="stat-value">{stats?.active_monitors || 0}</div>
            <div className="stat-label">Active Monitors</div>
            <div className="stat-subtitle">watching inbox</div>
          </div>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="quick-actions">
        <h2>Quick Actions</h2>
        <div className="actions-grid">
          <a href="/send-email" className="action-card">
            <div className="action-icon">📤</div>
            <div className="action-title">Send Email</div>
            <div className="action-description">Send a single email</div>
          </a>
          
          <a href="/campaigns" className="action-card">
            <div className="action-icon">📧</div>
            <div className="action-title">New Campaign</div>
            <div className="action-description">Create bulk email campaign</div>
          </a>
          
          <a href="/templates" className="action-card">
            <div className="action-icon">📝</div>
            <div className="action-title">Create Template</div>
            <div className="action-description">Design email template</div>
          </a>
          
          <a href="/contact-lists" className="action-card">
            <div className="action-icon">👥</div>
            <div className="action-title">Import Contacts</div>
            <div className="action-description">Add contacts from CSV</div>
          </a>
          
          <a href="/inbox-monitor" className="action-card">
            <div className="action-icon">🔔</div>
            <div className="action-title">Setup Monitor</div>
            <div className="action-description">Monitor inbox for keywords</div>
          </a>
          
          <a href="/scheduler" className="action-card">
            <div className="action-icon">⏰</div>
            <div className="action-title">Schedule Email</div>
            <div className="action-description">Schedule recurring emails</div>
          </a>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="recent-activity">
        <h2>Recent Activity</h2>
        {recentActivity.length === 0 ? (
          <div className="empty-activity">
            <p>No recent activity</p>
            <p>Start by sending an email or creating a campaign!</p>
          </div>
        ) : (
          <div className="activity-list">
            {recentActivity.map((activity) => (
              <div key={activity.id} className="activity-item">
                <div className="activity-icon">
                  {getActivityIcon(activity.activity_type)}
                </div>
                <div className="activity-content">
                  <div className="activity-description">{activity.description}</div>
                  <div className="activity-time">{formatTimestamp(activity.timestamp)}</div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default DashboardPage;