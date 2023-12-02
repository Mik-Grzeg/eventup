import React, { useState, useEffect } from 'react';
import axios from 'axios';

const ServiceList = ({ onMakeAppointment }) => {
  const [services, setServices] = useState([]);

  useEffect(() => {
    // Fetch services from the API
    const fetchServices = async () => {
      try {
        const response = await axios.get('http://localhost:8080/services');
        setServices(response.data);
      } catch (error) {
        console.error('Error fetching services:', error);
      }
    };

    fetchServices();
  }, []);

  return (
    <div>
      <h2>Available Services</h2>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Duration</th>
            <th>Price</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {services.map((service) => (
            <tr key={service.service_id}>
              <td>{service.service_id}</td>
              <td>{service.name}</td>
              <td>{service.duration}</td>
              <td>{service.price}</td>
              <td>
                <button onClick={() => onMakeAppointment(service.service_id)}>
                  Make Appointment
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default ServiceList;
