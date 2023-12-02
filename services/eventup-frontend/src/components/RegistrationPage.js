import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import axios from 'axios';

const RegistrationPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [phoneNumber, setPhoneNumber] = useState('');
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [registrationError, setRegistrationError] = useState(null);

  const handleRegister = async () => {
    try {
      const response = await axios.post('http://localhost:8080/api/v1/users', {
        email,
        password,
        phone_number: phoneNumber,
        first_name: firstName,
        last_name: lastName,
      });

      // Show success message to the user
      setRegistrationError('Registration successful!');

      // Optionally, you can redirect to the login page or handle it based on your application flow

      // Clear any previous registration error after a short delay
      setTimeout(() => {
        setRegistrationError(null);
      }, 5000);
    } catch (error) {
      // Handle registration error
      if (error.response) {
        setRegistrationError(`Registration failed: ${error.response.data}`);
      } else if (error.request) {
        setRegistrationError('Registration failed: No response received');
      } else {
        setRegistrationError(`Registration failed: ${error.message}`);
      }
    }
  };

  return (
    <div>
      <h1>Register</h1>
      {registrationError && <div style={{ color: registrationError.includes('successful') ? 'green' : 'red' }}>{registrationError}</div>}
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

      <div>
        Already registered? <Link to="/login">Login instead</Link>
      </div>
    </div>
  );
};

export default RegistrationPage;
