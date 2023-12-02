import React from 'react';
import ServiceList from './ServiceList';

const LandingPage = () => {
  const handleMakeAppointment = (serviceId) => {
    // Implement the logic to make an appointment using the serviceId
    console.log(`Make appointment for service ID: ${serviceId}`);
  };

  return (
    <div>
      <h1>Event Reservation App</h1>
      <ServiceList onMakeAppointment={handleMakeAppointment} />
    </div>
  );
};

export default LandingPage;
