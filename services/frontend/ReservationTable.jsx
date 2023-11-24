const { useState, useEffect } = React;

const MockedData = [
    { id: 1, data: '2023-01-01', godzina: '10:00 - 12:00', instruktor: 'Jan Kowalski' },
    { id: 2, data: '2023-01-02', godzina: '14:00 - 16:00', instruktor: 'Anna Nowak' },
    // Dodaj więcej danych, jeśli potrzebujesz
];

const ItemsPerPage = 5; // Liczba rekordów na stronę

const ReservationTable = () => {
    const [reservations, setReservations] = useState([]);
    const [currentPage, setCurrentPage] = useState(1);
    const maxPages = Math.ceil(MockedData.length / ItemsPerPage);

    useEffect(() => {
        // Symulacja pobierania danych z backendu
        setReservations(getCurrentPageData());
    }, [currentPage]);

    const getCurrentPageData = () => {
        const startIndex = (currentPage - 1) * ItemsPerPage;
        const endIndex = startIndex + ItemsPerPage;
        return MockedData.slice(startIndex, endIndex);
    };

    const handleReservation = (data, godzina, instruktor) => {
        alert(`Termin zarezerwowany:\nData: ${data}\nGodzina: ${godzina}\nInstruktor: ${instruktor}`);
        // Tutaj możesz dodać logikę do wysłania rezerwacji na backend
    };

    const handlePageChange = (newPage) => {
        setCurrentPage(newPage);
    };

    return (
        <div className="container">
            <h2 className="my-4">Najbliższe wolne terminy</h2>
            <table className="table">
                <thead>
                <tr>
                    <th>Data</th>
                    <th>Godzina</th>
                    <th>Instruktor</th>
                    <th>Rezerwuj</th>
                </tr>
                </thead>
                <tbody>
                {reservations.map((reservation) => (
                    <tr key={reservation.id}>
                        <td>{reservation.data}</td>
                        <td>{reservation.godzina}</td>
                        <td>{reservation.instruktor}</td>
                        <td>
                            <button
                                className="btn btn-primary"
                                onClick={() => handleReservation(reservation.data, reservation.godzina, reservation.instruktor)}
                            >
                                Rezerwuj
                            </button>
                        </td>
                    </tr>
                ))}
                </tbody>
            </table>

            {/* Paginacja */}
            <nav>
                <ul className="pagination">
                    {Array.from({ length: maxPages }, (_, i) => (
                        <li key={i + 1} className={`page-item ${currentPage === i + 1 ? 'active' : ''}`}>
                            <button className="page-link" onClick={() => handlePageChange(i + 1)}>
                                {i + 1}
                            </button>
                        </li>
                    ))}
                </ul>
            </nav>
        </div>
    );
};

ReactDOM.render(
    <ReservationTable />,
    document.getElementById('root')
);