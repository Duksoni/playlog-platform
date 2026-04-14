.PHONY: help start stop build rebuild logs init-network

ENV_FILE := .env.production
NETWORK := playlog_network

COMPOSE_USER_SERVICE := cd ./playlog-backend/services/user-service ; docker compose -p playlog-user --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/postgres/user/compose.yaml
COMPOSE_MULTIMEDIA_SERVICE := cd ./playlog-backend/services/multimedia-service ; docker compose -p playlog-multimedia --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/mongodb/multimedia/compose.yaml -f ../../docker/minio/compose.yaml
COMPOSE_CATALOGUE_SERVICE := cd ./playlog-backend/services/catalogue-service ; docker compose -p playlog-catalogue --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/postgres/catalogue/compose.yaml
COMPOSE_LIBRARY_SERVICE := cd ./playlog-backend/services/library-service ; docker compose -p playlog-library --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/postgres/library/compose.yaml
COMPOSE_REVIEW_SERVICE := cd ./playlog-backend/services/review-service ; docker compose -p playlog-review --env-file $(ENV_FILE) -f compose.yaml -f ../../docker/mongodb/review/compose.yaml
COMPOSE_API_GATEWAY := cd ./playlog-backend/services/api-gateway ; docker compose -p playlog-gateway --env-file $(ENV_FILE)
COMPOSE_FRONTEND := cd ./playlog-frontend ; docker compose -p playlog-frontend --env-file $(ENV_FILE)

DEV_COMPOSE_USER_SERVICE := $(COMPOSE_USER_SERVICE) -f ../../docker/postgres/user/compose.dev.yaml
DEV_COMPOSE_MULTIMEDIA_SERVICE := $(COMPOSE_MULTIMEDIA_SERVICE) -f ../../docker/mongodb/multimedia/compose.dev.yaml -f ../../docker/minio/compose.dev.yaml
DEV_COMPOSE_CATALOGUE_SERVICE := $(COMPOSE_CATALOGUE_SERVICE) -f ../../docker/postgres/catalogue/compose.dev.yaml
DEV_COMPOSE_LIBRARY_SERVICE := $(COMPOSE_LIBRARY_SERVICE) -f ../../docker/postgres/library/compose.dev.yaml
DEV_COMPOSE_REVIEW_SERVICE := $(COMPOSE_REVIEW_SERVICE) -f ../../docker/mongodb/review/compose.dev.yaml

help:
	@echo "Available targets:"
	@echo "  make start			- Start services"
	@echo "  make start-exposed		- Start services with exposed DB ports"
	@echo "  make stop			- Stop services"
	@echo "  make build			- Build images"
	@echo "  make rebuild [svc]		- Rebuild images (no cache)"
	@echo "  make logs [svc]		- Follow logs (optionally for a service)"
	@echo "  make init network		- Create shared network for services to use (one-time)"

start:
	@$(MAKE) init-network
	@$(COMPOSE_CATALOGUE_SERVICE) up -d
	@$(COMPOSE_USER_SERVICE) up -d
	@$(COMPOSE_MULTIMEDIA_SERVICE) up -d
	@$(COMPOSE_LIBRARY_SERVICE) up -d
	@$(COMPOSE_REVIEW_SERVICE) up -d
	@$(COMPOSE_API_GATEWAY) up -d
	@$(COMPOSE_FRONTEND) up -d
	@echo "  Access the app at:		http://localhost:8080";
	@echo "  Read OpenAPI Docs:		http://localhost:3000/docs";

start-exposed:
	@$(MAKE) init-network
	@$(DEV_COMPOSE_CATALOGUE_SERVICE) up -d
	@$(DEV_COMPOSE_USER_SERVICE) up -d
	@$(DEV_COMPOSE_MULTIMEDIA_SERVICE) up -d
	@$(DEV_COMPOSE_LIBRARY_SERVICE) up -d
	@$(DEV_COMPOSE_REVIEW_SERVICE) up -d
	@$(COMPOSE_API_GATEWAY) up -d
	@$(COMPOSE_FRONTEND) up -d
	@echo "  Access the app at:		http://localhost:8080";
	@echo "  Read OpenAPI Docs:		http://localhost:3000/docs";
	@echo "  Databases are exposed on their dev ports (5433-5435, 27018-27019, 9000-9001)";

stop:
	@$(COMPOSE_API_GATEWAY) down
	@$(COMPOSE_USER_SERVICE) down
	@$(COMPOSE_MULTIMEDIA_SERVICE) down
	@$(COMPOSE_CATALOGUE_SERVICE) down
	@$(COMPOSE_LIBRARY_SERVICE) down
	@$(COMPOSE_REVIEW_SERVICE) down
	@$(COMPOSE_FRONTEND) down

build:
	@$(MAKE) init-network
	@$(COMPOSE_USER_SERVICE) build
	@$(COMPOSE_MULTIMEDIA_SERVICE) build
	@$(COMPOSE_CATALOGUE_SERVICE) build
	@$(COMPOSE_LIBRARY_SERVICE) build
	@$(COMPOSE_REVIEW_SERVICE) build
	@$(COMPOSE_API_GATEWAY) build
	@$(COMPOSE_FRONTEND) build

rebuild:
	@$(MAKE) init-network
	@$(COMPOSE_USER_SERVICE) build --no-cache
	@$(COMPOSE_MULTIMEDIA_SERVICE) build --no-cache
	@$(COMPOSE_CATALOGUE_SERVICE) build --no-cache
	@$(COMPOSE_LIBRARY_SERVICE) build --no-cache
	@$(COMPOSE_REVIEW_SERVICE) build --no-cache
	@$(COMPOSE_API_GATEWAY) build --no-cache
	@$(COMPOSE_FRONTEND) build --no-cache

logs:
	@$(COMPOSE_API_GATEWAY) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_USER_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_MULTIMEDIA_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_CATALOGUE_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_LIBRARY_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_REVIEW_SERVICE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_FRONTEND) logs -f $(filter-out $@,$(MAKECMDGOALS))

init-network:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || docker network create --driver bridge $(NETWORK)

%:
	@:
