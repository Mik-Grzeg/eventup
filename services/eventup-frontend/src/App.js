import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { useAuth } from './hooks/AuthContext';  // Import useAuth
import LandingPage from './components/LandingPage';
import AdminDashboard from './components/AdminDashboard';
import LoginPage from './components/LoginPage';
import RegistrationPage from './components/RegistrationPage';
import UserDashboard from './components/UserDashboard';
import EmployeeDashboard from './components/EmployeeDashboard';

const App = () => {
  const { isAuthenticated, isAdmin, isRegularUser, isEmployee } = useAuth();

  return (
    <Router>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/register" element={<RegistrationPage />} />
        <Route
          path="/admin"
          element={isAdmin() ? <AdminDashboard /> : <Navigate to="/login" />}
        />
        <Route
          path="/dashboard"
          element={isRegularUser() ? <UserDashboard /> : <Navigate to="/login" />}
        />
        <Route
          path="/employee"
          element={isEmployee() ? <EmployeeDashboard /> : <Navigate to="/login" />}
        />        
      </Routes>
    </Router>
  );
};

export default App;
