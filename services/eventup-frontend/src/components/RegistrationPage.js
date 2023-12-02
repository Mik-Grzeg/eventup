import React, { useState } from 'react';
import { Link } from 'react-router-dom'; // Import Link from 'react-router-dom'
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
      const response = await axios.post('http://localhost:9080/api/v1/users', {
        email,
        password,
        phone_number: phoneNumber,
        first_name: firstName,
        last_name: lastName,
      });

      console.log('Registration successful. User:', response.data);
      // Optionally, you can redirect to the login page or handle it based on your application flow

      // Clear any previous registration error
      setRegistrationError(null);
    } catch (error) {
      console.error('Registration failed:', error);

      // Handle registration error
      if (error.response) {
        // The request was made and the server responded with a status code
        // that falls out of the range of 2xx
        console.error('Response data:', error.response.data);
        console.error('Response status:', error.response.status);
        console.error('Response headers:', error.response.headers);

        // Set the registration error state
        setRegistrationError(`Registration failed: ${error.response.data.message}`);
      } else if (error.request) {
        // The request was made but no response was received
        console.error('No response received:', error.request);

        // Set the registration error state
        setRegistrationError('Registration failed: No response received');
      } else {
        // Something happened in setting up the request that triggered an Error
        console.error('Error during request setup:', error.message);

        // Set the registration error state
        setRegistrationError(`Registration failed: ${error.message}`);
      }
    }
  };

  return (
    <div>
      <h1>Register</h1>
      {registrationError && <div style={{ color: 'red' }}>{registrationError}</div>}
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

      {/* Add a link to the login page */}
      <div>
        Already registered? <Link to="/login">Login instead</Link>
      </div>
    </div>
  );
};

export default RegistrationPage;
