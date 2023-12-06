import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useAuth } from '../hooks/AuthContext';
import Logout from './Logout';

const EmployeeDashboard = () => {
  const { logout, userId, token } = useAuth();
  const [appointments, setAppointments] = useState([]);
  const [shiftDetails, setShiftDetails] = useState({
    startShift: null,
    endShift: null,
  });
  const [newShiftDetails, setNewShiftDetails] = useState({
    employee_id: userId,
    service_id: '',
    start_shift: '',
    end_shift: '',
  });
  const [services, setServices] = useState([]);

  useEffect(() => {
    fetchAppointments();
    fetchShiftDetails();
    fetchServices();
  }, []);

  const fetchAppointments = async () => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/appointments/employee/${userId}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setAppointments(response.data);
    } catch (error) {
      console.error('Error fetching appointments:', error);
    }
  };

  const fetchShiftDetails = async () => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/employees/${userId}/shift`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setShiftDetails({
        startShift: response.data.start_shift,
        endShift: response.data.end_shift,
      });
    } catch (error) {
      console.error('Error fetching shift details:', error);
    }
  };

  const fetchServices = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/services');
      setServices(response.data);
    } catch (error) {
      console.error('Error fetching services:', error);
    }
  };

  const cancelAppointment = async (appointmentId) => {
    try {
      await axios.delete(`http://localhost:8080/api/v1/appointments/${appointmentId}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      fetchAppointments();
    } catch (error) {
      console.error('Error canceling appointment:', error);
    }
  };

  const editShiftDetails = async (newShiftDetails) => {
    try {
      await axios.put(`http://localhost:8080/api/v1/employees/${userId}/shift`, newShiftDetails, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      fetchShiftDetails();
    } catch (error) {
      console.error('Error editing shift details:', error);
    }
  };

  const createShiftDetails = async () => {
    try {
      await axios.post(`http://localhost:8080/api/v1/employees/${userId}/shift`, newShiftDetails, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      fetchShiftDetails();
    } catch (error) {
      console.error('Error creating shift details:', error);
    }
  };

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setNewShiftDetails((prevDetails) => ({ ...prevDetails, [name]: value }));
  };

  return (
    <div>
      <h1>Employee Dashboard</h1>

      {/* Display upcoming appointments */}
      <div>
        <h2>Upcoming Appointments</h2>
        <ul>
          {appointments.map((appointment) => (
            <li key={appointment.appointment_id}>
              {`${appointment.client_name} - ${appointment.start_time}`}
              <button onClick={() => cancelAppointment(appointment.appointment_id)}>
                Cancel Appointment
              </button>
            </li>
          ))}
        </ul>
      </div>

      {/* Display past appointments */}
      <div>
        <h2>Past Appointments</h2>
        <ul>{/* Similar to the upcoming appointments section */}</ul>
      </div>

      {/* Display and edit shift details */}
      <div>
        <h2>Shift Details</h2>
        <p>{`Start Shift: ${shiftDetails.startShift}`}</p>
        <p>{`End Shift: ${shiftDetails.endShift}`}</p>

        {/* Form for creating new shift details */}
        <form onSubmit={(e) => { e.preventDefault(); createShiftDetails(); }}>
          <label>
            Service:
            <select
              name="service_id"
              value={newShiftDetails.service_id}
              onChange={handleInputChange}
            >
              <option value="">Select a Service</option>
              {services.map((service) => (
                <option key={service.service_id} value={service.service_id}>
                  {service.name}
                </option>
              ))}
            </select>
          </label>
          <label>
            Start Shift:
            <input
              type="time"
              name="start_shift"
              value={newShiftDetails.start_shift}
              onChange={handleInputChange}
            />
          </label>
          <label>
            End Shift:
            <input
              type="time"
              name="end_shift"
              value={newShiftDetails.end_shift}
              onChange={handleInputChange}
            />
          </label>
          <button type="submit">Create Shift</button>
        </form>

        {/* Button for editing shift details */}
        <button onClick={() => editShiftDetails(newShiftDetails)}>Edit Shift</button>
      </div>

      {/* Your employee-specific content goes here */}
      <Logout />
    </div>
  );
};

export default EmployeeDashboard;
