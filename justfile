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
