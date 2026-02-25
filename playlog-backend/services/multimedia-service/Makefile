.PHONY: help start-dev start-db stop-dev rebuild-dev logs-dev

ENV_FILE := .env
NETWORK := playlog_network

COMPOSE_DEV_DB := docker compose -p playlog-multimedia-service-dev --env-file $(ENV_FILE) -f ../../docker/mongodb/multimedia/compose.yaml -f ../../docker/mongodb/multimedia/compose.dev.yaml -f ../../docker/minio/compose.yaml -f ../../docker/minio/compose.dev.yaml
COMPOSE_DEV := docker compose -p playlog-multimedia-service-dev --env-file $(ENV_FILE) -f ../../docker/mongo-express/compose.yaml -f ../../docker/mongodb/multimedia/compose.yaml -f ../../docker/mongo/multimedia/compose.dev.yaml -f ../../docker/minio/compose.yaml -f ../../docker/minio/compose.dev.yaml

help:
	@echo "Available targets:"
	@echo "  make start-dev      - Start dev services"
	@echo "  make start-db       - Start database service only"
	@echo "  make stop-dev       - Stop dev services"
	@echo "  make rebuild-dev    - Rebuild dev images (no cache)"
	@echo "  make logs-dev [svc] - Follow dev logs (optionally for a service)"

start-db:
	@$(MAKE) init-network
	@$(COMPOSE_DEV_DB) up -d
	@echo " Database will be available at localhost:27019"

start-dev:
	@$(MAKE) init-network
	@$(COMPOSE_DEV) up -d
	@$(MAKE) wait-dev

wait-dev:
	@echo "Waiting for multimedia-service deps to be healthy..."
	@bash -c '\
		SERVICES=(mongo_multimedia minio); \
		TIMEOUT=120; \
		INTERVAL=5; \
		ELAPSED=0; \
		while true; do \
			UNHEALTHY=(); \
			for service in "$${SERVICES[@]}"; do \
				ID=$$($(COMPOSE_DEV) ps -q "$$service"); \
				if [[ -z "$$ID" ]]; then \
					UNHEALTHY+=("$$service:missing"); \
					continue; \
				fi; \
				STATUS=$$(docker inspect --format="{{.State.Health.Status}}" "$$ID" 2>/dev/null || echo "unknown"); \
				if [[ "$$STATUS" != "healthy" ]]; then \
					UNHEALTHY+=("$$service:$$STATUS"); \
				fi; \
			done; \
			if [[ $${#UNHEALTHY[@]} -eq 0 ]]; then \
				break; \
			fi; \
			if (( ELAPSED >= TIMEOUT )); then \
				echo "Timeout waiting for services:"; \
				printf "  - %s\n" "$${UNHEALTHY[@]}"; \
				echo "Check logs with: make logs-dev [service]"; \
				exit 1; \
			fi; \
			printf "Waiting... (%ds/%ds) Unhealthy: %s\n" "$$ELAPSED" "$$TIMEOUT" "$${UNHEALTHY[*]}"; \
			sleep $$INTERVAL; \
			ELAPSED=$$((ELAPSED + INTERVAL)); \
		done; \
		echo ""; \
		echo "Multimedia service development environment is ready!"; \
		echo ""; \
		echo "Services:"; \
		echo "  - Mongo:			localhost:27019"; \
		echo "  - Mongo Express:		http://localhost:8085"; \
		echo "  - MinIO Console:   		http://localhost:9001"; \
		echo ""; \
		echo "View logs: make logs-dev [service_name]"; \
		echo "Stop: make stop-dev"; \
	'

init-network:
	@docker network inspect $(NETWORK) >/dev/null 2>&1 || docker network create --driver bridge $(NETWORK)

stop-dev:
	@$(COMPOSE_DEV) down

rebuild-dev:
	@$(MAKE) init-network
	@$(COMPOSE_DEV) build --no-cache

logs-dev:
	@$(COMPOSE_DEV) logs -f $(filter-out $@,$(MAKECMDGOALS))

%:
	@: