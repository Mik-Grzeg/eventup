import React from 'react';
import { useAuth } from '../hooks/AuthContext';
import Logout from './Logout';

const EmployeeDashboard = () => {
  const { logout } = useAuth();

  return (
    <div>
      <h1>Employee Dashboard</h1>
      <Logout />
      {/* Your employee-specific content goes here */}
    </div>
  );
};

export default EmployeeDashboard;
