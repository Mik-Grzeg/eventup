import React, { createContext, useContext, useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [token, setToken] = useState(() => {
    // Initialize token from sessionStorage or other secure storage
    return sessionStorage.getItem('token') || null;
  });

  const [userRole, setUserRole] = useState(null);

  const login = async (newToken) => {
    setToken(newToken);
    // Save the token to sessionStorage or other secure storage
    sessionStorage.setItem('token', newToken);

    // Fetch user role after login
    await fetchUserRole();

  };

  const logout = () => {
    setToken(null);
    setUserRole(null);
    // Remove the token from sessionStorage or other secure storage
    sessionStorage.removeItem('token');
  };

  const fetchUserRole = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/auth/access', {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      setUserRole(response.data.role);
    } catch (error) {
      console.error('Error fetching user role:', error);
    }
  };

  useEffect(() => {
    if (token) {
      fetchUserRole();
    }
  }, [token]);

  const isAdmin = () => userRole === 'admin';
  const isRegularUser = () => userRole === 'regular';

  return (
    <AuthContext.Provider value={{ token, userRole, login, logout, isAdmin, isRegularUser }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  return useContext(AuthContext);
};
