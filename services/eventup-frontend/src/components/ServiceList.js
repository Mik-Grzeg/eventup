// ServiceList.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Link } from 'react-router-dom';

const ServiceList = () => {
  const [services, setServices] = useState([]);

  useEffect(() => {
    // Fetch services from the API
    const fetchServices = async () => {
      try {
        const response = await axios.get('http://localhost:8080/api/v1/services');
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
            <th>Name</th>
            <th>Duration</th>
            <th>Price</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {services.map((service) => (
            <tr key={service.service_id}>
              <td>{service.name}</td>
              <td>{service.duration}</td>
              <td>{service.price}</td>
              <td>
                {/* Link to the AppointmentBookingPage with the selected service ID */}
                <Link to={`/appointment-booking/${service.service_id}`}>
                  <button>Make Appointment</button>
                </Link>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default ServiceList;
