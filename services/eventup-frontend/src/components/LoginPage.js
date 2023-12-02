import React, { useState } from 'react';
import { Link, Routes, useNavigate } from 'react-router-dom';
import { useAuth } from '../hooks/AuthContext'; // Update the import statement
import axios from 'axios';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const { login } = useAuth();
  const navigate = useNavigate();

  const handleLogin = async () => {
    try {
      const response = await axios.post('http://localhost:8080/api/v1/auth/login', { email, password });
      login(response.data.token);
      console.log('Login successful. Token:', response.data.token);
      // Redirect to the dashboard or handle it based on your application flow
      navigate('/UserDashboard');
    } catch (error) {
      console.error('Login failed:', error);
      // Handle login error (display an error message, etc.)
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
      <p>
        Don't have an account? <Link to="/register">Register here</Link>
      </p>

      {/* Define your routes here */}
      <Routes>
        {/* Example Route */}
        {/* <Route path="/example" element={<ExampleComponent />} /> */}
      </Routes>
    </div>
  );
};

export default LoginPage;
