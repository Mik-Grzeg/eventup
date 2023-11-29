import React, { useState } from 'react';
import axios from 'axios';

const RegistrationPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [phoneNumber, setPhoneNumber] = useState('');
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');

  const handleRegister = async () => {
    try {
      const response = await axios.post('http://localhost:9080/api/v1/users', { email, password, phone_number: phoneNumber, first_name: firstName, last_name: lastName }); // Replace with your actual API endpoint
      console.log('Registration successful. User:', response.data);
      // Redirect to the login page or handle it based on your application flow
    } catch (error) {
      console.error('Registration failed:', error);
      // Handle registration error (display an error message, etc.)
    }
  };

  return (
    <div>
      <h1>Register</h1>
      <div>
        <label>Email:</label>
        <input type="text" value={email} onChange={(e) => setEmail(e.target.value)} />
      </div>
      <div>
        <label>Password:</label>
        <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
      </div>
      <div>
        <label>Phone Number:</label>
        <input type="text" value={phoneNumber} onChange={(e) => setPhoneNumber(e.target.value)} />
      </div>
      <div>
        <label>First Name:</label>
        <input type="text" value={firstName} onChange={(e) => setFirstName(e.target.value)} />
      </div>
      <div>
        <label>Last Name:</label>
        <input type="text" value={lastName} onChange={(e) => setLastName(e.target.value)} />
      </div>
      <button onClick={handleRegister}>Register</button>
    </div>
  );
};

export default RegistrationPage;
