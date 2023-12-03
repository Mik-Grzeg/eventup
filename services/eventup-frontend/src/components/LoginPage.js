import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../hooks/AuthContext';
import axios from 'axios';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loginError, setLoginError] = useState(null);
  const { login, isAdmin, isRegularUser } = useAuth();
  const navigate = useNavigate();

  const handleLogin = async () => {
    try {
      const response = await axios.post('http://localhost:8080/api/v1/auth/login', { email, password });

      // Wait for the login function to complete before continuing
      const userRole = await login(response.data.token);

      console.log('Authentication successful. Token:', response.data.token);
      console.log('Authorization successful. Role:', userRole);

      // Navigate based on user role
      if (userRole === 'admin') {
        navigate('/admin');
        console.log('Navigating to /admin');
      } else if (userRole === 'regular') {
        navigate('/dashboard');
        console.log('Navigating to /dashboard');
      } else {
        navigate('/');
        console.log('Navigating to /');
      }
    } catch (error) {
      console.error('Login failed:', error);
      setLoginError('Incorrect username or password');
    }
  };

  return (
    <div>
      <h1>Login</h1>
      <div>
        <label>Email:</label>
        <input type="text" value={email} onChange={(e) => setEmail(e.target.value)} />
      </div>
      <div>
        <label>Password:</label>
        <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
      </div>
      <button onClick={handleLogin}>Login</button>
      {loginError && <div style={{ color: 'red' }}>{loginError}</div>}
      <p>
        Don't have an account? <Link to="/register">Register here</Link>
      </p>
    </div>
  );
};

export default LoginPage;
