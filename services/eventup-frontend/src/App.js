import React from 'react';
import { Router, Routes, Route } from 'react-router-dom';
import LandingPage from './components/LandingPage';
import AdminDashboard from './components/AdminDashboard';
import LoginPage from './components/LoginPage';
import RegistrationPage from './components/RegistrationPage';
import UserDashboard from './components/UserDashboard';
import { AuthProvider } from './hooks/AuthContext';
//import Protected from './components/Protected';

const App = () => {
  return (
    <Router>
      <AuthProvider>
        <Routes>
            <Route path="/" element={<LandingPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegistrationPage />} />
            <Route path="/dashboard" element={<UserDashboard />} />
            <Route path="/admin" element={<AdminDashboard />} />
            {/*<Protected path="/dashboard" element={<UserDashboard />} allowedRoles={['regular']} />
            <Protected path="/admin" element={<AdminDashboard />} allowedRoles={['admin']} />*/}
        </Routes>
      </AuthProvider>
    </Router>
  );
};

export default App;
