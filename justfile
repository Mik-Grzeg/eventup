set export
DATABASE_URL := "postgres://postgres:password@localhost:5432"

# start docker compose environment with default services
start:
  docker compose up -d --build

start-apps:
  docker compose up -d --build users-api
# delete all docker compose default services 
down:
  docker compose down -v
down-persist-volumes:
  docker compose down
# start docker compose environment using log profile which additionally enabled ClickHouse and RedPanda for application logging capabilities
start-log:
  docker compose --profile log up -d --build
# delete all docker compose services including those marked with log profile
down-log:
  docker compose --profile log down -v
# list containers
ps: 
  docker compose ps
# drops to postgresql shell
psql:
  docker compose exec postgres psql -U postgres

clean-tables:
  docker compose exec postgres psql -U postgres -c "TRUNCATE TABLE user_log_infos CASCADE"
  docker compose exec postgres psql -U postgres -c "TRUNCATE TABLE user_accounts CASCADE"
  docker compose exec postgres psql -U postgres -c "TRUNCATE TABLE appointments CASCADE"
  docker compose exec postgres psql -U postgres -c "TRUNCATE TABLE employee_schedules CASCADE"
  docker compose exec postgres psql -U postgres -c "TRUNCATE TABLE services CASCADE"
  

demo-data:
  docker compose exec postgres psql -U postgres -c "$(cat ./services/users/tests/fixtures/users.sql)"
  docker compose exec postgres psql -U postgres -c "$(cat ./services/appointments/tests/fixtures/services.sql)"
  docker compose exec postgres psql -U postgres -c "$(cat ./services/appointments/tests/fixtures/schedules.sql)"
  docker compose exec postgres psql -U postgres -c "$(cat ./services/appointments/tests/fixtures/appointments.sql)"
