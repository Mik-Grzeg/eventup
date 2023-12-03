import React from 'react';
import { Route, Navigate } from 'react-router-dom';
import { useAuth } from '../hooks/AuthContext';

const Protected = ({ element, allowedRoles }) => {
  const { isAdmin, isRegularUser } = useAuth();

  // Redirect to login if the user is not authenticated
  if (!isAdmin() && !isRegularUser()) {
    return <Navigate to="/login" />;
  }

  // Redirect to home if the user doesn't have the required role
  if (allowedRoles.includes('admin') && !isAdmin()) {
    return <Navigate to="/" />;
  }

  if (allowedRoles.includes('regular') && !isRegularUser()) {
    return <Navigate to="/dashboard" />;
  }

  // Render the protected component using the Route component
  return <Route element={element} />;
};

export default Protected;
