import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useAuth } from '../hooks/AuthContext';
import Logout from './Logout';
import EmployeeManagement from './EmployeeManagement'; 

const ServiceTable = ({ services, toggleServiceVisibility, handleEditService }) => {
  const { isAdmin } = useAuth();

  return (
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Description</th>
          <th>Duration</th>
          <th>Price</th>
          <th>Active</th>
          <th>Action</th>
        </tr>
      </thead>
      <tbody>
        {services.map((service) => (
          <tr key={service.service_id}>
            <td>{service.name}</td>
            <td>{service.description}</td>
            <td>{service.duration_in_sec}</td>
            <td>{service.price}</td>
            <td>
              <input
                type="checkbox"
                checked={service.active}
                onChange={() => toggleServiceVisibility(service)}
                disabled={!isAdmin()}
              />
            </td>
            <td>
              <button onClick={() => handleEditService(service)}>Edit</button>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

const AdminDashboard = () => {
  const { token } = useAuth();
  const [services, setServices] = useState([]);
  const [selectedService, setSelectedService] = useState({
    name: '',
    description: '',
    duration_in_sec: 0,
    price: 0,
    active: true,
  });
  const [isEditing, setIsEditing] = useState(false);
  const [serviceModalVisible, setServiceModalVisible] = useState(false);

  useEffect(() => {
    fetchServices();
  }, []);

  const fetchServices = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/services', {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setServices(response.data);
    } catch (error) {
      console.error('Error fetching services:', error);
    }
  };

  const handleCreateService = async () => {
    try {
      const numericService = {
        ...selectedService,
        duration_in_sec: Number(selectedService.duration_in_sec),
        price: Number(selectedService.price),
      };
      await axios.post('http://localhost:8080/api/v1/services', numericService, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setServiceModalVisible(false);
      fetchServices();
    } catch (error) {
      console.error('Error creating service:', error);
    }
  };

  const handleUpdateService = async () => {
    try {
      const numericService = {
        ...selectedService,
        duration_in_sec: Number(selectedService.duration_in_sec),
        price: Number(selectedService.price),
      };
      await axios.put(`http://localhost:8080/api/v1/services/${selectedService.service_id}`, numericService, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setServiceModalVisible(false);
      setIsEditing(false);
      fetchServices();
    } catch (error) {
      console.error('Error updating service:', error);
    }
  };

  const toggleServiceVisibility = async (service) => {
    try {
      const updatedService = { ...service, active: !service.active };
      setSelectedService(updatedService);
      await axios.put(`http://localhost:8080/api/v1/services/${service.service_id}`, updatedService, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      fetchServices();
    } catch (error) {
      console.error('Error toggling service visibility:', error);
    }
  };

  const handleEditService = (service) => {
    const { duration_in_sec, price, ...rest } = service;
    setSelectedService({
      ...rest,
      duration_in_sec: duration_in_sec.toString(),
      price: price.toString(),
    });
    setServiceModalVisible(true);
    setIsEditing(true);
  };

  return (
    <div>
      <h1>Admin Dashboard</h1>
      <Logout />
      <button onClick={() => { setSelectedService({ name: '', description: '', duration_in_sec: 0, price: 0, active: true }); setIsEditing(false); setServiceModalVisible(true); }}>Add Service</button>

      {/* Display a list of services */}
      <ServiceTable
        services={services}
        toggleServiceVisibility={toggleServiceVisibility}
        handleEditService={handleEditService}
      />

      {/* Service Modal */}
      {serviceModalVisible && (
        <div>
          <h3>{isEditing ? 'Edit Service' : 'Add Service'}</h3>
          {/* Input fields for creating/editing service */}
          <label>
            Name:
            <input
              type="text"
              value={selectedService.name}
              onChange={(e) => setSelectedService({ ...selectedService, name: e.target.value })}
            />
          </label>
          <label>
            Description:
            <input
              type="text"
              value={selectedService.description}
              onChange={(e) => setSelectedService({ ...selectedService, description: e.target.value })}
            />
          </label>
          <label>
            Duration (in seconds):
            <input
              type="number"
              value={selectedService.duration_in_sec}
              onChange={(e) => setSelectedService({ ...selectedService, duration_in_sec: e.target.value })}
            />
          </label>
          <label>
            Price:
            <input
              type="number"
              value={selectedService.price}
              onChange={(e) => setSelectedService({ ...selectedService, price: e.target.value })}
            />
          </label>

          <button onClick={isEditing ? handleUpdateService : handleCreateService}>
            Save
          </button>
          <button onClick={() => setServiceModalVisible(false)}>Cancel</button>
        </div>
      )}
      <EmployeeManagement />
    </div>
  );
};

export default AdminDashboard;
