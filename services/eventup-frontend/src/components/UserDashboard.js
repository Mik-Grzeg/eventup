import React, { useState, useEffect } from 'react';
import axios from 'axios';

const UserDashboard = () => {
  const [services, setServices] = useState([]);
  const [serviceModalVisible, setServiceModalVisible] = useState(false);
  const [selectedService, setSelectedService] = useState(null);

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

  const handleCreateService = (newService) => {
    axios.post('http://localhost:8080/api/v1/services', newService)
      .then(response => {
        setServices([...services, response.data]);
        setServiceModalVisible(false);
      })
      .catch(error => {
        console.error('Error creating service:', error);
      });
  };

  const handleUpdateService = (updatedService) => {
    axios.put(`http://localhost:8080/api/v1/services/${updatedService.service_id}`, updatedService)
      .then(response => {
        const updatedServices = services.map(service =>
          service.service_id === updatedService.service_id ? response.data : service
        );
        setServices(updatedServices);
        setServiceModalVisible(false);
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
    setSelectedService(service);
    setServiceModalVisible(true);
  };

  const closeServiceModal = () => {
    setSelectedService(null);
    setServiceModalVisible(false);
  };

  return (
    <div>
      <h1>User Dashboard</h1>

      <button onClick={() => openServiceModal(null)}>Add Service</button>

      {/* Display a list of services */}
      <h2>Services</h2>
      <ul>
        {services.map(service => (
          <li key={service.service_id}>
            {service.name} - Duration: {service.duration}, Price: {service.price}
            <button onClick={() => openServiceModal(service)}>Edit</button>
            <button onClick={() => handleRemoveService(service.service_id)}>Remove</button>
          </li>
        ))}
      </ul>

      {/* Service Modal */}
      {serviceModalVisible && (
        <div>
          <h3>{selectedService ? 'Edit Service' : 'Add Service'}</h3>
          <label>Name:</label>
          <input
            type="text"
            value={selectedService ? selectedService.name : ''}
            onChange={(e) => setSelectedService({ ...selectedService, name: e.target.value })}
          />
          {/* Add more input fields for other service properties */}
          <button onClick={() => selectedService ? handleUpdateService(selectedService) : handleCreateService(selectedService)}>Save</button>
          <button onClick={closeServiceModal}>Cancel</button>
        </div>
      )}
    </div>
  );
};

export default UserDashboard;
