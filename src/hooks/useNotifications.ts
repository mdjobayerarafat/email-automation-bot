import { useState } from 'react';

export const useNotifications = () => {
  const [error, setError] = useState<string>('');
  const [success, setSuccess] = useState<string>('');

  const showError = (message: string) => {
    setError(message);
    setTimeout(() => setError(''), 5000);
  };

  const showSuccess = (message: string) => {
    setSuccess(message);
    setTimeout(() => setSuccess(''), 5000);
  };

  const clearError = () => setError('');
  const clearSuccess = () => setSuccess('');
  const clearAll = () => {
    setError('');
    setSuccess('');
  };

  return {
    error,
    success,
    showError,
    showSuccess,
    clearError,
    clearSuccess,
    clearAll
  };
};