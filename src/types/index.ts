// User types
export interface User {
  id: number;
  username: string;
  email: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

// Email types
export interface EmailAccount {
  id: number;
  user_id: number;
  account_name: string;
  email_address: string;
  username: string;
  smtp_server: string;
  smtp_port: number;
  imap_server: string;
  imap_port: number;
  is_active: boolean;
  created_at: string;
}

export interface EmailTemplate {
  id: number;
  user_id: number;
  name: string;
  subject: string;
  html_content: string;
  text_content?: string;
  variables: string[];
  created_at: string;
}

export interface EmailStats {
  total_sent: number;
  total_received: number;
  total_failed: number;
  success_rate: number;
}

// Automation types
export interface AutomationRule {
  id: number;
  user_id: number;
  name: string;
  trigger_type: string;
  conditions: any;
  actions: any;
  is_active: boolean;
  created_at: string;
}

// Form types
export interface LoginForm {
  email: string;
  password: string;
}

export interface RegisterForm {
  username: string;
  email: string;
  password: string;
}

export interface EmailAccountForm {
  account_name: string;
  email_address: string;
  username: string;
  password: string;
  smtp_server: string;
  smtp_port: number;
  imap_server: string;
  imap_port: number;
}

export interface TemplateForm {
  name: string;
  subject: string;
  html_content: string;
  text_content: string;
  variables: string;
}

export interface EmailForm {
  to: string;
  subject: string;
  html_body: string;
  text_body: string;
}

// Dashboard types
export interface DashboardStats {
  total_emails: number;
  sent_today: number;
  active_rules: number;
  success_rate: number;
}