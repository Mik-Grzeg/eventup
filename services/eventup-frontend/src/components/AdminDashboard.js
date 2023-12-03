import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Logout from './Logout';

const AdminDashboard = () => {
  const [services, setServices] = useState([]);
  const [serviceModalVisible, setServiceModalVisible] = useState(false);
  const [selectedService, setSelectedService] = useState({
    company_id: '',
    name: '',
    description: '',
    duration_in_sec: 0,
    price: 0,
  });

  useEffect(() => {
    // Fetch services when the component mounts
    axios.get('http://localhost:8080/api/v1/services')
      .then(response => {
        setServices(response.data);
      })
      .catch(error => {
        console.error('Error fetching services:', error);
      });
  }, []);

  const handleCreateService = () => {
    axios.post('http://localhost:8080/api/v1/services', selectedService)
      .then(response => {
        setServices([...services, response.data]);
        setServiceModalVisible(false);
        clearSelectedService();
      })
      .catch(error => {
        console.error('Error creating service:', error);
      });
  };

  const handleUpdateService = () => {
    axios.put(`http://localhost:8080/api/v1/services/${selectedService.service_id}`, selectedService)
      .then(response => {
        const updatedServices = services.map(service =>
          service.service_id === selectedService.service_id ? response.data : service
        );
        setServices(updatedServices);
        setServiceModalVisible(false);
        clearSelectedService();
      })
      .catch(error => {
        console.error('Error updating service:', error);
      });
  };

  const handleRemoveService = (serviceId) => {
    axios.delete(`http://localhost:8080/api/v1/services/${serviceId}`)
      .then(() => {
        const updatedServices = services.filter(service => service.service_id !== serviceId);
        setServices(updatedServices);
      })
      .catch(error => {
        console.error('Error removing service:', error);
      });
  };

  const openServiceModal = (service) => {
    setSelectedService(service || {
      company_id: '',
      name: '',
      description: '',
      duration_in_sec: 0,
      price: 0,
    });
    setServiceModalVisible(true);
  };

  const closeServiceModal = () => {
    setServiceModalVisible(false);
    clearSelectedService();
  };

  const clearSelectedService = () => {
    setSelectedService({
      company_id: '',
      name: '',
      description: '',
      duration_in_sec: 0,
      price: 0,
    });
  };

  return (
    <div>
      <h1>Admin Dashboard</h1>
      <Logout />      
      <button onClick={() => openServiceModal(null)}>Add Service</button>

      {/* Display a list of services */}
      <h2>Services</h2>
      <ul>
        {services.map(service => (
          <li key={service.service_id}>
            {service.name} - Duration: {service.duration_in_sec}s, Price: {service.price}
            <button onClick={() => openServiceModal(service)}>Edit</button>
            <button onClick={() => handleRemoveService(service.service_id)}>Remove</button>
          </li>
        ))}
      </ul>

      {/* Service Modal */}
      {serviceModalVisible && (
        <div>
          <h3>{selectedService.service_id ? 'Edit Service' : 'Add Service'}</h3>
          <label>Name:</label>
          <input
            type="text"
            value={selectedService.name}
            onChange={(e) => setSelectedService({ ...selectedService, name: e.target.value })}
          />
          <label>Description:</label>
          <input
            type="text"
            value={selectedService.description}
            onChange={(e) => setSelectedService({ ...selectedService, description: e.target.value })}
          />
          <label>Duration (seconds):</label>
          <input
            type="number"
            value={selectedService.duration_in_sec}
            onChange={(e) => setSelectedService({ ...selectedService, duration_in_sec: +e.target.value })}
          />
          <label>Price:</label>
          <input
            type="number"
            value={selectedService.price}
            onChange={(e) => setSelectedService({ ...selectedService, price: +e.target.value })}
          />
          <button onClick={() => selectedService.service_id ? handleUpdateService() : handleCreateService()}>Save</button>
          <button onClick={closeServiceModal}>Cancel</button>
        </div>
      )}
    </div>
  );
};

export default AdminDashboard;
