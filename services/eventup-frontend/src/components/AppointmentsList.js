import React, { useState, useEffect } from 'react';
import axios from 'axios';

const EventList = () => {
  const [events, setEvents] = useState([]);
  const [currentPage, setCurrentPage] = useState(1);

  useEffect(() => {
    // Fetch events from your backend API
    const fetchEvents = async () => {
      try {
        const response = await axios.get('/api/v1/appointments'); // Replace with your actual API endpoint
        setEvents(response.data); // Assuming your API returns an array of events
      } catch (error) {
        console.error('Error fetching events:', error);
      }
    };

    fetchEvents();
  }, []);

  // Paginate events
  const eventsPerPage = 50;
  const indexOfLastEvent = currentPage * eventsPerPage;
  const indexOfFirstEvent = indexOfLastEvent - eventsPerPage;
  const currentEvents = events.slice(indexOfFirstEvent, indexOfLastEvent);

  // Change page
  const paginate = (pageNumber) => setCurrentPage(pageNumber);

  return (
    <div>
      <h2>Events</h2>
      <ul>
        {currentEvents.map((event) => (
          <li key={event.event_id}>{event.name}</li>
        ))}
      </ul>
      <div>
        {/* Pagination controls */}
        {Array.from({ length: Math.ceil(events.length / eventsPerPage) }).map((_, index) => (
          <button key={index} onClick={() => paginate(index + 1)}>
            {index + 1}
          </button>
        ))}
      </div>
    </div>
  );
};

export default EventList;
