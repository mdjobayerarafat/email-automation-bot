import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { UserInfo, LoginResponse, LoginForm, RegisterForm } from '../types';

export const useAuth = () => {
  const [user, setUser] = useState<UserInfo | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [token, setToken] = useState<string | null>(null);

  // Check authentication on mount
  useEffect(() => {
    const checkAuth = async () => {
      const storedToken = localStorage.getItem('auth_token');
      if (storedToken) {
        try {
          const response = await invoke<UserInfo>('verify_token', { token: storedToken });
          setUser(response);
          setIsAuthenticated(true);
          setToken(storedToken);
        } catch (error) {
          console.error('Token verification failed:', error);
          localStorage.removeItem('auth_token');
          setIsAuthenticated(false);
          setToken(null);
        }
      }
      setIsLoading(false);
    };

    checkAuth();
  }, []);

  const login = async (loginData: LoginForm): Promise<void> => {
    setIsLoading(true);
    try {
      const response = await invoke<LoginResponse>('login_user', { loginData });
      setUser(response.user);
      setIsAuthenticated(true);
      setToken(response.token);
      localStorage.setItem('auth_token', response.token);
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (userData: RegisterForm): Promise<void> => {
    setIsLoading(true);
    try {
      const response = await invoke<LoginResponse>('register_user', { userData });
      setUser(response.user);
      setIsAuthenticated(true);
      setToken(response.token);
      localStorage.setItem('auth_token', response.token);
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = () => {
    localStorage.removeItem('auth_token');
    setUser(null);
    setIsAuthenticated(false);
    setToken(null);
  };

  return {
    user,
    isAuthenticated,
    isLoading,
    token,
    login,
    register,
    logout
  };
};