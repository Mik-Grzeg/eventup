// components/UserDashboard.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useAuth } from '../hooks/AuthContext';
import { Link } from 'react-router-dom';
import Logout from './Logout';

const UserDashboard = () => {
  const { token, userId } = useAuth();
  const [appointments, setAppointments] = useState([]);

  const fetchAppointments = async () => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/appointments/${userId}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setAppointments(response.data);
    } catch (error) {
      console.error('Error fetching appointments:', error);
    }
  };

  useEffect(() => {
    fetchAppointments();
  }, [token]);

  const endAppointment = async (appointmentId) => {
    try {
      await axios.post(
        `http://localhost:8080/api/v1/appointments/${appointmentId}/end`,
        {},
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );
      // Update the list of appointments after ending one
      fetchAppointments();
    } catch (error) {
      console.error('Error ending appointment:', error);
    }
  };

  return (
    <div>
      <h2>User Dashboard</h2>
      <Logout />
      <p>
        Looking for service? Go to{' '}
        <Link to="/">list of all available services</Link>
      </p>
      <table>
        <thead>
          <tr>
            <th>Appointment ID</th>
            <th>Client Name</th>
            <th>Start Time</th>
            <th>End Time</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {appointments.map((appointment) => (
            <tr key={appointment.appointment_id}>
              <td>{appointment.appointment_id}</td>
              <td>{appointment.client_name}</td>
              <td>{appointment.start_time}</td>
              <td>{appointment.end_time}</td>
              <td>
                <button onClick={() => endAppointment(appointment.appointment_id)}>
                  End Appointment
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default UserDashboard;
