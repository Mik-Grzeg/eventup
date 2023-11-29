# Appointments API

## Endpoints

### `/api/v1/appointments`

```curl
curl localhost:9081/api/v1/appointments -H "Authorization: Bearer $(curl -X POST localhost:9080/api/v1/auth/login -d '{"email": "jjjj@doe.com", "password": "dodooooo"}' -H 'Content-Type: application/json' | jq -r '.token')" -v

```
