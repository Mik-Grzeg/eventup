import React, { createContext, useContext, useState, useEffect } from 'react';
import axios from 'axios';

const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [token, setToken] = useState(() => {
    return sessionStorage.getItem('token') || null;
  });

  const [userRole, setUserRole] = useState(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const login = async (newToken) => {
    setToken(newToken);
    sessionStorage.setItem('token', newToken);
    setIsAuthenticated(true);

    // Return user role
    return fetchUserRole(newToken);
  };

  const logout = () => {
    setToken(null);
    setUserRole(null);
    setIsAuthenticated(false);
    sessionStorage.removeItem('token');
  };

  const fetchUserRole = async (currentToken) => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/auth/access', {
        headers: {
          Authorization: `Bearer ${currentToken}`,
        },
      });
      setUserRole(response.data.role);
      return response.data.role;
    } catch (error) {
      console.error('Error fetching user role:', error);
      return null;
    }
  };

  useEffect(() => {
    const fetchData = async () => {
      if (token) {
        await fetchUserRole(token);
      }
    };

    fetchData();
  }, [token]);

  const isAdmin = () => userRole === 'admin';
  const isRegularUser = () => userRole === 'regular';

  return (
    <AuthContext.Provider value={{ token, userRole, isAuthenticated, login, logout, isAdmin, isRegularUser }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  return useContext(AuthContext);
};
