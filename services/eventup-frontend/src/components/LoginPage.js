import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../hooks/AuthContext';
import axios from 'axios';
import { useLocation } from 'react-router-dom';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loginError, setLoginError] = useState(null);
  const { login, isAdmin, isRegularUser, isEmployee } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();  // Change this line

  const handleLogin = async () => {
    try {
      const response = await axios.post('http://rest.yuadgroup.uk/api/v1/auth/login', { email, password });
      const userRole = await login(response.data.token);

      console.log('Authentication successful. Token:', response.data.token);
      console.log('Authorization successful. Role:', userRole);

      // Get the intended redirect path from the state
      const intendedRedirect = location.state?.intendedRedirect;

      // Navigate based on user role or intended redirect path
      if (userRole === 'admin') {
        navigate('/admin');
        console.log('Navigating to /admin');
      } else if (userRole === 'regular') {
        navigate('/dashboard');
        console.log('Navigating to /dashboard');
      } else if (userRole === 'employee') {
        navigate('/employee');
        console.log('Navigating to /employee');
      } else {
        navigate(intendedRedirect || '/');
        console.log(`Navigating to ${intendedRedirect || '/'}`);
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
