// User types
export interface User {
  id: number;
  username: string;
  email: string;
  password_hash: string;
  created_at: string;
  updated_at: string;
}

export interface CreateUser {
  username: string;
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: UserInfo;
}

export interface UserInfo {
  id: number;
  username: string;
  email: string;
}

// Email Account types
export interface EmailAccount {
  id: number;
  user_id: number;
  account_name: string;
  email_address: string;
  imap_server?: string;
  imap_port?: number;
  smtp_server?: string;
  smtp_port?: number;
  username: string;
  password_encrypted: string;
  is_active: boolean;
  created_at: string;
}

export interface CreateEmailAccount {
  account_name: string;
  email_address: string;
  imap_server?: string;
  imap_port?: number;
  smtp_server?: string;
  smtp_port?: number;
  username: string;
  password: string;
}

// Email Template types
export interface EmailTemplate {
  id: number;
  user_id: number;
  name: string;
  subject?: string;
  body?: string;
  template_type?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateEmailTemplate {
  name: string;
  subject?: string;
  body?: string;
  template_type?: string;
}

// Automation Rule types
export interface AutomationRule {
  id: number;
  user_id: number;
  rule_name: string;
  keywords: string[];
  conditions: any;
  actions: any;
  is_active: boolean;
  created_at: string;
}

export interface CreateAutomationRule {
  rule_name: string;
  keywords: string[];
  conditions: any;
  actions: any;
}

// Email Log types
export interface EmailLog {
  id: number;
  user_id: number;
  email_account_id?: number;
  direction: string; // 'sent' or 'received'
  recipient_email?: string;
  sender_email?: string;
  subject?: string;
  status: string;
  error_message?: string;
  sent_at?: string;
  created_at: string;
}

export interface CreateEmailLog {
  user_id: number;
  email_account_id?: number;
  direction: string;
  recipient_email?: string;
  sender_email?: string;
  subject?: string;
  status: string;
  error_message?: string;
  sent_at?: string;
}

// Scheduled Email types
export interface ScheduledEmail {
  id: number;
  user_id: number;
  template_id?: number;
  recipient_list: string[];
  scheduled_time: string;
  recurrence_pattern?: string;
  status: string; // 'pending', 'sent', 'failed'
  created_at: string;
}

export interface CreateScheduledEmail {
  template_id?: number;
  recipient_list: string[];
  scheduled_time: string;
  recurrence_pattern?: string;
}

// Email Message types
export interface EmailMessage {
  to: string[];
  cc?: string[];
  bcc?: string[];
  subject: string;
  body: string;
  attachments?: string[];
}

export interface BatchEmailRequest {
  template_id: number;
  recipients: RecipientData[];
  schedule_time?: string;
}

export interface RecipientData {
  email: string;
  variables: Record<string, string>;
}

// Stats types
export interface EmailStats {
  total_sent: number;
  total_received: number;
  total_failed: number;
  automation_rules_count: number;
  success_rate: number;
}

export interface ConnectionTest {
  success: boolean;
  message: string;
}

// Form types (for UI components)
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
  smtp_server?: string;
  smtp_port?: number;
  imap_server?: string;
  imap_port?: number;
}

export interface TemplateForm {
  name: string;
  subject?: string;
  body?: string;
  template_type?: string;
}

export interface EmailForm {
  to: string[];
  cc?: string[];
  bcc?: string[];
  subject: string;
  body: string;
  html_body?: string;
  text_body?: string;
  attachments?: string[];
}

export interface AutomationRuleForm {
  rule_name: string;
  keywords: string;
  conditions: string;
  actions: string;
}

// Dashboard types
export interface DashboardStats {
  total_emails_sent: number;
  total_emails_failed: number;
  total_contacts: number;
  total_templates: number;
  total_campaigns: number;
  active_monitors: number;
}