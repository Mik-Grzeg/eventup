import React from 'react';
import { useAuth } from '../hooks/AuthContext';
import { useNavigate } from 'react-router-dom';

const Logout = () => {
  const { logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/');
  };

  return (
    <div style={{ position: 'absolute', top: 10, right: 10 }}>
      <button onClick={handleLogout}>Logout</button>
    </div>
  );
};

export default Logout;
