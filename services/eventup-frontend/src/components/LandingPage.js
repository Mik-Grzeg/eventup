// LandingPage.js
import React from 'react';
import ServiceList from './ServiceList';
import NavigationMenu from './NavigationMenu';
import { Link } from 'react-router-dom';

const LandingPage = () => {
  const handleMakeAppointment = (serviceId) => {
    // Implement the logic to make an appointment using the serviceId
    console.log(`Make appointment for service ID: ${serviceId}`);
  };

  return (
    <div>
      <header>
        <h1>Event Reservation App</h1>
        <NavigationMenu />
      </header>
      <ServiceList onMakeAppointment={handleMakeAppointment} />
    </div>
  );
};

export default LandingPage;
