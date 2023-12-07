# Appointment Management API

This API provides endpoints for managing appointments, services, and employee schedules.


## Base URL

- Local: `http://localhost:8080`
- Secure Local: `https://localhost:443`

## Examplar API calls

### Get services
Retrieve details of all services.

```curl
curl http://localhost:8080/api/v1/services
```

### Get service details

```curl
curl -X GET http://localhost:8080/api/v1/services/{service_id}
```

### Create service.

```curl
curl -X POST -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d '{
  "name": "Service Name",
  "description": "Service Description",
  "duration_in_sec": 1800,
  "price": 50.00,
  "active": true
}' http://localhost:8080/api/v1/services
```

### Update service

```curl
curl -X PUT -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d '{
  "name": "Updated Service Name",
  "description": "Updated Service Description",
  "price": 75.00,
}' http://localhost:8080/api/v1/services/{service_id}
```

### GET free slots for a service for given day
 
```curl
curl -X GET http://localhost:8080/api/v1/services/{service_id}/appointments/free?date=2023-12-06
```

### Get appointment details

```curl
curl -X GET -H "Authorization: Bearer $TOKEN" http://localhost:8080/api/v1/appointments/{appointment_id}
```

### Create appointment

```curl
curl -X POST -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d '{
  "client_id": "client_id_value",
  "employee_id": "employee_id_value",
  "service_id": "service_id_value",
  "client_name": "Client Name",
  "start_time": "2023-12-07T10:00:00",
  "end_time": "2023-12-07T11:00:00"
}' http://localhost:8080/api/v1/appointments
```

### Cancel appointment

```curl
curl -X PUT -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d '{
  "reason": "I changed my mind"
}' http://localhost:8080/api/v1/appointments/{appointment_id}/cancel
```

### Serve appointment

```curl
curl -X PUT -H "Authorization: Bearer $TOKEN" http://localhost:8080/api/v1/appointments/{appointment_id}/serve
```

### Get employee schedules

```curl
curl -X GET http://localhost:8080/api/v1/employee_schedules
```

### Create employee schedule

```curl
curl -X POST -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d '{
  "employee_id": "employee_id_value",
  "service_id": "service_id_value",
  "start_shift": "2023-12-07T11:00:00",
  "end_shift": "2023-12-07T18:00:00"
}' http://localhost:8080/api/v1/employee_schedules
```
