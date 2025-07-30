import React from 'react';
import { useTheme } from '../contexts/ThemeContext';

interface SidebarProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
  currentUser: any;
  onLogout: () => void;
}

const Sidebar: React.FC<SidebarProps> = ({ activeTab, setActiveTab, currentUser, onLogout }) => {
  const { theme, toggleTheme } = useTheme();

  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: '📊' },
    { id: 'accounts', label: 'Email Accounts', icon: '📧' },
    { id: 'templates', label: 'Templates', icon: '📝' },
    { id: 'automation', label: 'Automation', icon: '⚡' },
    { id: 'compose', label: 'Compose', icon: '✍️' },
    { id: 'documentation', label: 'Documentation', icon: '📚' },
  ];

  return (
    <div className="sidebar">
      <div className="sidebar-header">
        <div className="app-logo">
          <span className="logo-icon">📮</span>
          <h2>Email Bot</h2>
        </div>
      </div>

      <nav className="sidebar-nav">
        {menuItems.map((item) => (
          <button
            key={item.id}
            className={`nav-item ${activeTab === item.id ? 'active' : ''}`}
            onClick={() => setActiveTab(item.id)}
          >
            <span className="nav-icon">{item.icon}</span>
            <span className="nav-label">{item.label}</span>
          </button>
        ))}
      </nav>

      <div className="sidebar-footer">
        <div className="theme-toggle">
          <button onClick={toggleTheme} className="theme-btn">
            <span className="theme-icon">{theme === 'light' ? '🌙' : '☀️'}</span>
            <span className="theme-label">{theme === 'light' ? 'Dark' : 'Light'}</span>
          </button>
        </div>
        
        <div className="user-section">
          <div className="user-info">
            <div className="user-avatar">
              {currentUser?.username?.charAt(0).toUpperCase() || 'U'}
            </div>
            <div className="user-details">
              <div className="username">{currentUser?.username}</div>
              <div className="user-email">{currentUser?.email}</div>
            </div>
          </div>
          <button onClick={onLogout} className="logout-btn">
            <span>🚪</span>
          </button>
        </div>
      </div>
    </div>
  );
};

export default Sidebar;