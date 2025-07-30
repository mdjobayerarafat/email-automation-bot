import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { User, LoginResponse, LoginForm, RegisterForm } from '../types';

export const useAuth = () => {
  const [user, setUser] = useState<User | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  // Check authentication on mount
  useEffect(() => {
    const checkAuth = async () => {
      const token = localStorage.getItem('auth_token');
      if (token) {
        try {
          const response = await invoke<User>('verify_token', { token });
          setUser(response);
          setIsAuthenticated(true);
        } catch (error) {
          console.error('Token verification failed:', error);
          localStorage.removeItem('auth_token');
          setIsAuthenticated(false);
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
  };

  return {
    user,
    isAuthenticated,
    isLoading,
    login,
    register,
    logout
  };
};