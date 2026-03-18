.PHONY: help start stop build rebuild logs init-network

ENV_FILE := .env.production
NETWORK := playlog_network

COMPOSE_USER_SERVICE := cd ./playlog-backend/services/user-service ; docker compose --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/postgres/user/compose.yaml
COMPOSE_MULTIMEDIA_SERVICE := cd ./playlog-backend/services/multimedia-service; docker compose --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/mongodb/multimedia/compose.yaml -f ../../docker/minio/compose.yaml
COMPOSE_CATALOGUE_SERVICE := cd ./playlog-backend/services/catalogue-service; docker compose --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/postgres/catalogue/compose.yaml
COMPOSE_API_GATEWAY := cd ./playlog-backend/services/api-gateway ; docker compose --env-file $(ENV_FILE)
COMPOSE_FRONTEND := cd ./playlog-frontend ; docker compose --env-file $(ENV_FILE)

help:
	@echo "Available targets:"
	@echo "  make start			- Start dev services"
	@echo "  make stop			- Stop dev services"
	@echo "  make build			- Build images"
	@echo "  make rebuild [svc]		- Rebuild images (no cache)"
	@echo "  make logs [svc]		- Follow logs (optionally for a service)"
	@echo "  make init network		- Create shared network for services to use (one-time)"

start:
	@$(MAKE) init-network
	@$(COMPOSE_CATALOGUE_SERVICE) up -d
	@$(COMPOSE_USER_SERVICE) up -d
	@$(COMPOSE_MULTIMEDIA_SERVICE) up -d
	@$(COMPOSE_API_GATEWAY) up -d
	@$(COMPOSE_FRONTEND) up -d
	@echo "  Access the app at:		http://localhost:8080";
	@echo "  Read OpenAPI Docs:		http://localhost:3000/docs";

stop:
	@$(COMPOSE_API_GATEWAY) down
	@$(COMPOSE_USER_SERVICE) down
	@$(COMPOSE_MULTIMEDIA_SERVICE) down
	@$(COMPOSE_CATALOGUE_SERVICE) down
	@$(COMPOSE_FRONTEND) down

build:
	@$(MAKE) init-network
	@$(COMPOSE_USER_SERVICE) build
	@$(COMPOSE_MULTIMEDIA_SERVICE) build
	@$(COMPOSE_CATALOGUE_SERVICE) build
	@$(COMPOSE_API_GATEWAY) build
	@$(COMPOSE_FRONTEND) build

rebuild:
	@$(MAKE) init-network
	@$(COMPOSE_USER_SERVICE) build --no-cache
	@$(COMPOSE_MULTIMEDIA_SERVICE) build --no-cache
	@$(COMPOSE_CATALOGUE_SERVICE) build --no-cache
	@$(COMPOSE_API_GATEWAY) build --no-cache
	@$(COMPOSE_FRONTEND) build --no-cache

logs:
	@$(COMPOSE_API_GATEWAY) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_USER_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_MULTIMEDIA_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_CATALOGUE_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_FRONTEND) logs -f $(filter-out $@,$(MAKECMDGOALS))

init-network:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || docker network create --driver bridge $(NETWORK)

%:
	@: