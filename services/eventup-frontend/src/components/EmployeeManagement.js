// EmployeeDEmployeeManagement.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useAuth } from '../hooks/AuthContext';

const EmployeeManagement = () => {
  const { token } = useAuth();
  const [employees, setEmployees] = useState([]);
  const [selectedEmployee, setSelectedEmployee] = useState({
    email: '',
    password: '',
    phone_number: '',
    first_name: '',
    last_name: '',
  });
  const [isEditing, setIsEditing] = useState(false);
  const [employeeModalVisible, setEmployeeModalVisible] = useState(false);

  useEffect(() => {
    fetchEmployees();
  }, []);

  const fetchEmployees = async () => {
    try {
      const response = await axios.get('http://rest.yuadgroup.uk/api/v1/users/employees', {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setEmployees(response.data);
    } catch (error) {
      console.error('Error fetching employees:', error);
    }
  };

  const handleCreateEmployee = async () => {
    try {
      const newEmployee = {
        ...selectedEmployee,
        role: 'employee', // Add the role field
      };

      await axios.post('http://rest.yuadgroup.uk/api/v1/users', newEmployee, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      setEmployeeModalVisible(false);
      fetchEmployees();
    } catch (error) {
      console.error('Error creating employee:', error);
    }
  };

  const handleUpdateEmployee = async () => {
    try {
      await axios.put(`http://rest.yuadgroup.uk/api/v1/users/${selectedEmployee.user_id}`, selectedEmployee, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      setEmployeeModalVisible(false);
      setIsEditing(false);
      fetchEmployees();
    } catch (error) {
      console.error('Error updating employee:', error);
    }
  };

  const handleDeleteEmployee = async (employeeId) => {
    try {
      await axios.delete(`http://rest.yuadgroup.uk/api/v1/users/${employeeId}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      fetchEmployees();
    } catch (error) {
      console.error('Error deleting employee:', error);
    }
  };

  const handleEditEmployee = (employee) => {
    setSelectedEmployee(employee);
    setEmployeeModalVisible(true);
    setIsEditing(true);
  };

  return (
    <div>
      <h2>Employee Management</h2>
      <button onClick={() => { setSelectedEmployee({ email: '', password: '', phone_number: '', first_name: '', last_name: '' }); setIsEditing(false); setEmployeeModalVisible(true); }}>Add Employee</button>

      {/* Display a list of employees */}
      <table>
        <thead>
          <tr>
            <th>Email</th>
            <th>First Name</th>
            <th>Last Name</th>
            <th>Phone Number</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {employees.map((employee) => (
            <tr key={employee.user_id}>
              <td>{employee.email}</td>
              <td>{employee.first_name}</td>
              <td>{employee.last_name}</td>
              <td>{employee.phone_number}</td>
              <td>
                <button onClick={() => handleEditEmployee(employee)}>Edit</button>
                <button onClick={() => handleDeleteEmployee(employee.user_id)}>Delete</button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* Employee Modal */}
      {employeeModalVisible && (
        <div>
          <h3>{isEditing ? 'Edit Employee' : 'Add Employee'}</h3>
          {/* Input fields for creating/editing employee */}
          <label>
            Email:
            <input
              type="text"
              value={selectedEmployee.email}
              onChange={(e) => setSelectedEmployee({ ...selectedEmployee, email: e.target.value })}
            />
          </label>
          <label>
            Password:
            <input
              type="password"
              value={selectedEmployee.password}
              onChange={(e) => setSelectedEmployee({ ...selectedEmployee, password: e.target.value })}
            />
          </label>
          <label>
            First Name:
            <input
              type="text"
              value={selectedEmployee.first_name}
              onChange={(e) => setSelectedEmployee({ ...selectedEmployee, first_name: e.target.value })}
            />
          </label>
          <label>
            Last Name:
            <input
              type="text"
              value={selectedEmployee.last_name}
              onChange={(e) => setSelectedEmployee({ ...selectedEmployee, last_name: e.target.value })}
            />
          </label>
          <label>
            Phone Number:
            <input
              type="text"
              value={selectedEmployee.phone_number}
              onChange={(e) => setSelectedEmployee({ ...selectedEmployee, phone_number: e.target.value })}
            />
          </label>

          <button onClick={isEditing ? handleUpdateEmployee : handleCreateEmployee}>
            Save
          </button>
          <button onClick={() => setEmployeeModalVisible(false)}>Cancel</button>
        </div>
      )}
    </div>
  );
};

export default EmployeeManagement;
