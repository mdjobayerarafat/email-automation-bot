import React from 'react';

interface NotificationBarProps {
  error?: string;
  success?: string;
}

const NotificationBar: React.FC<NotificationBarProps> = ({ error, success }) => {
  if (!error && !success) return null;

  return (
    <>
      {error && <div className="error">{error}</div>}
      {success && <div className="success">{success}</div>}
    </>
  );
};

export default NotificationBar;