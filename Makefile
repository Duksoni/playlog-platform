.PHONY: help start start-exposed stop build rebuild logs init-network

ENV_FILE := .env.production
NETWORK := playlog_network

# --- Service Compose Commands ---
COMPOSE_USER       := cd ./playlog-backend/services/user-service && docker compose -p playlog-user --env-file $$(ENV_FILE) -f compose.yaml -f ../../docker/postgres/user/compose.yaml
COMPOSE_MULTIMEDIA := cd ./playlog-backend/services/multimedia-service && docker compose -p playlog-multimedia --env-file $$(ENV_FILE) -f compose.yaml -f ../../docker/mongodb/multimedia/compose.yaml -f ../../docker/minio/compose.yaml
COMPOSE_CATALOGUE  := cd ./playlog-backend/services/catalogue-service && docker compose -p playlog-catalogue --env-file $$(ENV_FILE) -f compose.yaml -f ../../docker/postgres/catalogue/compose.yaml
COMPOSE_LIBRARY    := cd ./playlog-backend/services/library-service && docker compose -p playlog-library --env-file $$(ENV_FILE) -f compose.yaml -f ../../docker/postgres/library/compose.yaml
COMPOSE_REVIEW     := cd ./playlog-backend/services/review-service && docker compose -p playlog-review --env-file $$(ENV_FILE) -f compose.yaml -f ../../docker/mongodb/review/compose.yaml
COMPOSE_GATEWAY    := cd ./playlog-backend/services/api-gateway && docker compose -p playlog-gateway --env-file $$(ENV_FILE)
COMPOSE_FRONTEND   := cd ./playlog-frontend && docker compose -p playlog-frontend --env-file $$(ENV_FILE)

# Dev variants with exposed ports
DEV_USER           := $(COMPOSE_USER) -f ../../docker/postgres/user/compose.dev.yaml
DEV_MULTIMEDIA     := $(COMPOSE_MULTIMEDIA) -f ../../docker/mongodb/multimedia/compose.dev.yaml -f ../../docker/minio/compose.dev.yaml
DEV_CATALOGUE      := $(COMPOSE_CATALOGUE) -f ../../docker/postgres/catalogue/compose.dev.yaml
DEV_LIBRARY        := $(COMPOSE_LIBRARY) -f ../../docker/postgres/library/compose.dev.yaml
DEV_REVIEW         := $(COMPOSE_REVIEW) -f ../../docker/mongodb/review/compose.dev.yaml

# --- Global Targets ---

help:
	@echo "Available targets:"
	@echo "  make start			- Start all services"
	@echo "  make start-exposed		- Start all services with exposed DB ports"
	@echo "  make stop			- Stop all services"
	@echo "  make build			- Build all images"
	@echo "  make rebuild			- Rebuild all images (no cache)"
	@echo "  make logs [svc]		- Follow logs (optionally for a specific project)"
	@echo ""
	@echo "Service-specific targets ([svc] can be: user-service, multimedia-service, catalogue-service, library-service, review-service, api-gateway, frontend):"
	@echo "  make start-[svc]"
	@echo "  make stop-[svc]"
	@echo "  make restart-[svc]"
	@echo "  make build-[svc]"
	@echo "  make rebuild-[svc]"
	@echo "  make logs-[svc]"

start: init-network start-catalogue-service start-user-service start-multimedia-service start-library-service start-review-service start-api-gateway start-frontend
	@echo "  Access the app at:		http://localhost:8080";
	@echo "  Read OpenAPI Docs:		http://localhost:3000/docs";

start-exposed: init-network start-exposed-catalogue-service start-exposed-user-service start-exposed-multimedia-service start-exposed-library-service start-exposed-review-service start-api-gateway start-frontend
	@echo "  Access the app at:		http://localhost:8080";
	@echo "  Read OpenAPI Docs:		http://localhost:3000/docs";
	@echo "  Databases are exposed on their dev ports (5433-5435, 27018-27019, 9000-9001)";

stop: stop-api-gateway stop-user-service stop-multimedia-service stop-catalogue-service stop-library-service stop-review-service stop-frontend

build: build-user-service build-multimedia-service build-catalogue-service build-library-service build-review-service build-api-gateway build-frontend

rebuild: rebuild-user-service rebuild-multimedia-service rebuild-catalogue-service rebuild-library-service rebuild-review-service rebuild-api-gateway rebuild-frontend

define SERVICE_TARGETS
.PHONY: start-$(1) stop-$(1) restart-$(1) build-$(1) rebuild-$(1) logs-$(1)
start-$(1): init-network
	@$(2) up -d
stop-$(1):
	@$(2) down
restart-$(1):
	@$(2) down
	@$(2) up -d
build-$(1):
	@$(2) build
rebuild-$(1):
	@$(2) build --no-cache
logs-$(1):
	@$(2) logs -f
endef

$(eval $(call SERVICE_TARGETS,user-service,$(COMPOSE_USER)))
$(eval $(call SERVICE_TARGETS,multimedia-service,$(COMPOSE_MULTIMEDIA)))
$(eval $(call SERVICE_TARGETS,catalogue-service,$(COMPOSE_CATALOGUE)))
$(eval $(call SERVICE_TARGETS,library-service,$(COMPOSE_LIBRARY)))
$(eval $(call SERVICE_TARGETS,review-service,$(COMPOSE_REVIEW)))
$(eval $(call SERVICE_TARGETS,api-gateway,$(COMPOSE_GATEWAY)))
$(eval $(call SERVICE_TARGETS,frontend,$(COMPOSE_FRONTEND)))

# Special for start-exposed individual targets
.PHONY: start-exposed-user-service start-exposed-multimedia-service start-exposed-catalogue-service start-exposed-library-service start-exposed-review-service
start-exposed-user-service: init-network
	@$(DEV_USER) up -d
start-exposed-multimedia-service: init-network
	@$(DEV_MULTIMEDIA) up -d
start-exposed-catalogue-service: init-network
	@$(DEV_CATALOGUE) up -d
start-exposed-library-service: init-network
	@$(DEV_LIBRARY) up -d
start-exposed-review-service: init-network
	@$(DEV_REVIEW) up -d

# --- Helpers ---

logs:
	@$(COMPOSE_GATEWAY) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_CATALOGUE) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_USER) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_MULTIMEDIA) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_LIBRARY) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_REVIEW) logs -f $(filter-out $@,$(MAKECMDGOALS))
	@$(COMPOSE_FRONTEND) logs -f $(filter-out $@,$(MAKECMDGOALS))

init-network:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || docker network create --driver bridge $(NETWORK)

%:
	@:
