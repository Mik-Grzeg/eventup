// components/ServiceList.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Link, useNavigate } from 'react-router-dom';
import DatePicker from 'react-datepicker';
import 'react-datepicker/dist/react-datepicker.css';
import { useAuth } from '../hooks/AuthContext'; 

const ServiceList = () => {
  const [services, setServices] = useState([]);
  const [selectedService, setSelectedService] = useState(null);
  const [selectedServiceName, setSelectedServiceName] = useState(null);
  const [selectedDate, setSelectedDate] = useState(new Date());
  const [availableSlots, setAvailableSlots] = useState([]);
  const [employeeAppointments, setEmployeeAppointments] = useState({});
  const [selectedEmployee, setSelectedEmployee] = useState(null);
  const [selectedTimeRange, setSelectedTimeRange] = useState(null);
  const [intendedRedirect, setIntendedRedirect] = useState(null); // State to store the intended redirect path
  const {
    token,
    isAuthenticated,
    userId,
    userEmail,
    userFirstName,
    userLastName,
    userPhoneNumber,
    fetchUserDetails,
    // ... (other AuthContext properties)
  } = useAuth();

  const navigate = useNavigate();

  const fetchServices = async () => {
    try {
      const response = await axios.get('http://rest.yuadgroup.uk/api/v1/services');
      setServices(response.data);
    } catch (error) {
      console.error('Error fetching services:', error);
    }
  };

  const fetchAppointments = async () => {
    try {
      const response = await axios.get('http://rest.yuadgroup.uk/api/v1/appointments');
      setEmployeeAppointments(response.data);
    } catch (error) {
      console.error('Error fetching appointments:', error);
    }
  };

  useEffect(() => {
    fetchServices();
    fetchAppointments();
  }, []);

  const handleShowSlots = async (serviceId) => {
    if (!serviceId) {
      console.error('Please select a service');
      return;
    }
    
    const formattedDate = selectedDate.toISOString().split('T')[0];

    try {
      const response = await axios.get(
        `http://rest.yuadgroup.uk/api/v1/services/${serviceId}/appointments/free`,
        {
          params: { date: formattedDate },
        }
      );

      setAvailableSlots(response.data);
    } catch (error) {
      console.error('Error fetching available slots:', error);
    }
  };

  const handleMakeAppointment = async () => {
    if (!selectedEmployee || !selectedTimeRange) {
      console.error('Please select employee and time range');
      return;
    }

    // Check if the user is logged in
    if (!isAuthenticated) {
      // If not logged in, set intended redirect path and redirect to login page
      navigate('/login', { state: { intendedRedirect: '/appointments' } });
      return;
    }

    // Logic to make an appointment with the selected data
    try {
      // Fetch additional user details if authenticated
      if (isAuthenticated) {
        await fetchUserDetails(userId);
      }

      const appointmentData = {
        service_id: selectedService,
        client_id: userId,
        employee_id: selectedEmployee,
        client_name: `${userFirstName} ${userLastName}`,
        start_time: selectedTimeRange.split(' - ')[0],
        end_time: selectedTimeRange.split(' - ')[1],
      };

      // Make an appointment with the selected data
      const response = await axios.post('http://rest.yuadgroup.uk/api/v1/appointments', appointmentData, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      console.log('Appointment created successfully:', response.data);

      // Redirect to the dashboard or any other page after successful appointment creation
      navigate('/dashboard');
    } catch (error) {
      console.error('Error creating appointment:', error);
    }
  };


  return (
    <div>
      <h2>List of Available Services</h2>
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Description</th>
            <th>Duration (min)</th>
            <th>Price</th>
            <th>Date</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {services.map((service) => (
            <tr key={service.service_id}>
              <td>{service.name}</td>
              <td>{service.description}</td>
              <td>{Math.floor(service.duration_in_sec / 60)}</td>
              <td>${service.price}</td>
              <td>
                <DatePicker selected={selectedDate} onChange={(date) => setSelectedDate(date)} />
              </td>
              <td>
                <button onClick={() => {
                  handleShowSlots(service.service_id);
                  setSelectedService(service.service_id); // Set the selected service
                  setSelectedServiceName(service.name)
                }}>
                  Show Slots
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* Display available slots for each employee */}
      {availableSlots.length > 0 && (
        <div>
          <h3>{`Available slots for ${selectedServiceName}`}</h3>
          <table>
            <thead>
              <tr>
                <th>Employee ID</th>
                <th>Date</th>
                <th>Time Range</th>
                <th>Action</th>
              </tr>
            </thead>
            <tbody>
              {Array.from(new Set(availableSlots.map((slot) => slot.employee_id))).map(
                (employeeId) => (
                  <tr key={employeeId}>
                    <td>{employeeId}</td>
                    <td>{new Date(availableSlots[0].slot_start_time).toLocaleDateString()}</td>
                    <td>
                      {/* Render dropdown with available slots for the selected employee */}
                      <select
                        onChange={(e) => {
                          setSelectedEmployee(employeeId);
                          setSelectedTimeRange(e.target.value);
                        }}
                      >
                        {availableSlots
                          .filter((s) => s.employee_id === employeeId)
                          .map((s) => (
                            <option key={s.slot_start_time} value={`${s.slot_start_time} - ${s.slot_end_time}`}>
                              {`${new Date(s.slot_start_time).toLocaleTimeString([], {
                                hour: '2-digit',
                                minute: '2-digit',
                              })} - ${new Date(s.slot_end_time).toLocaleTimeString([], {
                                hour: '2-digit',
                                minute: '2-digit',
                              })}`}
                            </option>
                          ))}
                      </select>
                    </td>
                    <td>
                      <button onClick={handleMakeAppointment}>Make Appointment</button>
                    </td>
                  </tr>
                )
              )}
            </tbody>
          </table>
        </div>
      )}

      <p>
        <Link to="/dashboard">Go to User Dashboard</Link>
      </p>
    </div>
  );
};

export default ServiceList;
