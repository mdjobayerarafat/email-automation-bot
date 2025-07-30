import React, { useState, useEffect } from 'react';
import Lottie from 'lottie-react';
import { LoginForm, RegisterForm } from '../types';
import { useAuth } from '../hooks/useAuth';
import { useNotifications } from '../hooks/useNotifications';
import loginAnimation from '../assets/Login.json';

interface AuthPageProps {
  onAuthSuccess: () => void;
}

const AuthPage: React.FC<AuthPageProps> = ({ onAuthSuccess }) => {
  const [isLogin, setIsLogin] = useState(true);
  const [loginForm, setLoginForm] = useState<LoginForm>({ email: '', password: '' });
  const [registerForm, setRegisterForm] = useState<RegisterForm>({ 
    username: '', 
    email: '', 
    password: '' 
  });
  const [isAnimating, setIsAnimating] = useState(false);
  const [focusedField, setFocusedField] = useState<string | null>(null);
  
  const { login, register, isLoading } = useAuth();
  const { error, success, showError, showSuccess } = useNotifications();

  useEffect(() => {
    setIsAnimating(true);
    const timer = setTimeout(() => setIsAnimating(false), 500);
    return () => clearTimeout(timer);
  }, []);

  const handleTabSwitch = (loginMode: boolean) => {
    setIsAnimating(true);
    setTimeout(() => {
      setIsLogin(loginMode);
      setIsAnimating(false);
    }, 150);
  };

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await login(loginForm);
      showSuccess('Login successful!');
      onAuthSuccess();
    } catch (err) {
      showError(err as string);
    }
  };

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await register(registerForm);
      showSuccess('Registration successful!');
      onAuthSuccess();
    } catch (err) {
      showError(err as string);
    }
  };

  return (
    <div className="auth-container">
      <div className="auth-background-animation">
        <div className="floating-shape shape-1"></div>
        <div className="floating-shape shape-2"></div>
        <div className="floating-shape shape-3"></div>
      </div>
      
      <div className={`auth-card ${isAnimating ? 'animating' : ''}`}>
        <div className="auth-left-panel">
          <div className="lottie-animation">
            <Lottie 
              animationData={loginAnimation} 
              loop={true}
              style={{ width: 200, height: 200 }}
            />
          </div>
          <div className="auth-branding">
            <h2>Welcome Back!</h2>
            <p>Sign in to continue your email automation journey</p>
          </div>
        </div>
        
        <div className="auth-right-panel">
          <div className="auth-header">
            <h1>Email Automation Bot</h1>
            <p>Automate your email workflows with ease</p>
          </div>
        
        <div className="auth-tabs">
          <div className="tab-slider" style={{ transform: `translateX(${isLogin ? '0%' : '100%'})` }}></div>
          <button 
            className={`tab-button ${isLogin ? 'active' : ''}`}
            onClick={() => handleTabSwitch(true)}
          >
            <span>Login</span>
          </button>
          <button 
            className={`tab-button ${!isLogin ? 'active' : ''}`}
            onClick={() => handleTabSwitch(false)}
          >
            <span>Register</span>
          </button>
        </div>

        <div className={`form-container ${isAnimating ? 'form-animating' : ''}`}>
          {isLogin ? (
            <form onSubmit={handleLogin} className="auth-form">
              <div className="input-group">
                <input
                  type="email"
                  id="login-email"
                  value={loginForm.email}
                  onChange={(e) => setLoginForm({...loginForm, email: e.target.value})}
                  onFocus={() => setFocusedField('login-email')}
                  onBlur={() => setFocusedField(null)}
                  required
                  className={loginForm.email ? 'has-value' : ''}
                />
                <label htmlFor="login-email" className={focusedField === 'login-email' ? 'focused' : ''}>
                  <span>📧</span> Email Address
                </label>
                <div className="input-border"></div>
              </div>
              
              <div className="input-group">
                <input
                  type="password"
                  id="login-password"
                  value={loginForm.password}
                  onChange={(e) => setLoginForm({...loginForm, password: e.target.value})}
                  onFocus={() => setFocusedField('login-password')}
                  onBlur={() => setFocusedField(null)}
                  required
                  className={loginForm.password ? 'has-value' : ''}
                />
                <label htmlFor="login-password" className={focusedField === 'login-password' ? 'focused' : ''}>
                  <span>🔒</span> Password
                </label>
                <div className="input-border"></div>
              </div>
              
              <button type="submit" disabled={isLoading} className="submit-btn">
                <span className="btn-content">
                  {isLoading ? (
                    <>
                      <div className="loading-spinner"></div>
                      Logging in...
                    </>
                  ) : (
                    <>
                      <span>🚀</span>
                      Login
                    </>
                  )}
                </span>
              </button>
            </form>
          ) : (
            <form onSubmit={handleRegister} className="auth-form">
              <div className="input-group">
                <input
                  type="text"
                  id="register-username"
                  value={registerForm.username}
                  onChange={(e) => setRegisterForm({...registerForm, username: e.target.value})}
                  onFocus={() => setFocusedField('register-username')}
                  onBlur={() => setFocusedField(null)}
                  required
                  className={registerForm.username ? 'has-value' : ''}
                />
                <label htmlFor="register-username" className={focusedField === 'register-username' ? 'focused' : ''}>
                  <span>👤</span> Username
                </label>
                <div className="input-border"></div>
              </div>
              
              <div className="input-group">
                <input
                  type="email"
                  id="register-email"
                  value={registerForm.email}
                  onChange={(e) => setRegisterForm({...registerForm, email: e.target.value})}
                  onFocus={() => setFocusedField('register-email')}
                  onBlur={() => setFocusedField(null)}
                  required
                  className={registerForm.email ? 'has-value' : ''}
                />
                <label htmlFor="register-email" className={focusedField === 'register-email' ? 'focused' : ''}>
                  <span>📧</span> Email Address
                </label>
                <div className="input-border"></div>
              </div>
              
              <div className="input-group">
                <input
                  type="password"
                  id="register-password"
                  value={registerForm.password}
                  onChange={(e) => setRegisterForm({...registerForm, password: e.target.value})}
                  onFocus={() => setFocusedField('register-password')}
                  onBlur={() => setFocusedField(null)}
                  required
                  className={registerForm.password ? 'has-value' : ''}
                />
                <label htmlFor="register-password" className={focusedField === 'register-password' ? 'focused' : ''}>
                  <span>🔒</span> Password
                </label>
                <div className="input-border"></div>
              </div>
              
              <button type="submit" disabled={isLoading} className="submit-btn">
                <span className="btn-content">
                  {isLoading ? (
                    <>
                      <div className="loading-spinner"></div>
                      Creating account...
                    </>
                  ) : (
                    <>
                      <span>✨</span>
                      Create Account
                    </>
                  )}
                </span>
              </button>
            </form>
          )}
        </div>
        
          {error && (
            <div className="notification error-notification">
              <span className="notification-icon">⚠️</span>
              <span className="notification-text">{error}</span>
            </div>
          )}
          {success && (
            <div className="notification success-notification">
              <span className="notification-icon">✅</span>
              <span className="notification-text">{success}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default AuthPage;