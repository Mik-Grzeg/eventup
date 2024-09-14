import React, { createContext, useContext, useState, useEffect } from 'react';
import axios from 'axios';

const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [token, setToken] = useState(() => {
    return sessionStorage.getItem('token') || null;
  });

  const [userRole, setUserRole] = useState(null);
  const [userId, setUserId] = useState(null);
  const [userEmail, setUserEmail] = useState(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [userFirstName, setUserFirstName] = useState(null);
  const [userLastName, setUserLastName] = useState(null);
  const [userPhoneNumber, setUserPhoneNumber] = useState(null);

  const login = async (newToken) => {
    setToken(newToken);
    sessionStorage.setItem('token', newToken);
    setIsAuthenticated(true);

    // Return user role
    return fetchUserRole(newToken);
  };

  const logout = () => {
    setToken(null);
    setUserId(null);
    setUserRole(null);
    setUserEmail(null);
    setIsAuthenticated(false);
    sessionStorage.removeItem('token');
  };

  const fetchUserRole = async (currentToken) => {
    try {
      const response = await axios.get('http://rest.yuadgroup.uk/api/v1/auth/access', {
        headers: {
          Authorization: `Bearer ${currentToken}`,
        },
      });
      setUserRole(response.data.role);
      setUserId(response.data.id);
      setUserEmail(response.data.email)
      setIsLoading(false); // Set loading to false when user role is fetched
      return response.data.role;
    } catch (error) {
      console.error('Error fetching user role:', error);
      setIsLoading(false); // Set loading to false in case of an error
      return null;
    }
  };

  const fetchUserDetails = async (userId) => {
    try {
      const response = await axios.get(`http://rest.yuadgroup.uk/api/v1/users/${userId}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      // Set user details from the response
      setUserId(response.data.user_id);
      setUserEmail(response.data.email);
      setUserFirstName(response.data.first_name);
      setUserLastName(response.data.last_name);
      setUserPhoneNumber(response.data.phone_number);

      return response.data; // Return the entire user details object if needed
    } catch (error) {
      console.error('Error fetching user details:', error);
      return null;
    }
  };

  useEffect(() => {
    const fetchData = async () => {
      // Check if there's a token in session storage
      const storedToken = sessionStorage.getItem('token');
      if (storedToken) {
        // If there's a stored token, set it in the state
        setToken(storedToken);
        // Set isAuthenticated to true
        setIsAuthenticated(true);
        // Fetch user role
        await fetchUserRole(storedToken);
      } else {
        setIsLoading(false); // Set loading to false if there's no stored token
      }
    };

    fetchData();
  }, []); // Empty dependency array ensures this runs once on mount

  const isAdmin = () => userRole === 'admin';
  const isRegularUser = () => userRole === 'regular';
  const isEmployee = () => userRole === 'employee';



  return (
    <AuthContext.Provider
      value={{
        token,
        userRole,
        userId,
        userEmail,
        userFirstName,
        userLastName,
        userPhoneNumber,
        isAuthenticated,
        isLoading,
        login,
        logout,
        isAdmin,
        isRegularUser,
        isEmployee,
        fetchUserDetails,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  return useContext(AuthContext);
};