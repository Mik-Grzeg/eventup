import React, { useState } from 'react';
import { Link, Routes, Route } from 'react-router-dom'; // Update import statement

import axios from 'axios';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = async () => {
    try {
      const response = await axios.post('http://localhost:9080/api/v1/auth/login', { email, password }); // Replace with your actual API endpoint
      // Save the token in localStorage or a state management solution
      console.log('Login successful. Token:', response.data.token);
      // Redirect to the admin dashboard or handle it based on your application flow
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
