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

  const cancelAppointment = async (appointmentId) => {
    try {
      await axios.put(
        `http://localhost:8080/api/v1/appointments/${appointmentId}/cancel`,
        { reason: "Your cancellation reason here" }, // Replace with the actual reason for cancellation
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );
      // Update the list of appointments after canceling one
      fetchAppointments();
    } catch (error) {
      console.error('Error canceling appointment:', error);
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
            <th>Client Name</th>
            <th>Date</th>
            <th>Time Range</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {appointments.map((appointment) => (
            <tr key={appointment.appointment_id}>
              <td>{appointment.client_name}</td>
              <td>{new Date(appointment.start_time).toLocaleDateString()}</td>
              <td>{`${new Date(appointment.start_time).toLocaleTimeString()} - ${new Date(appointment.end_time).toLocaleTimeString()}`}</td>
              <td>
                <button onClick={() => cancelAppointment(appointment.appointment_id)}>
                  Cancel Appointment
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
