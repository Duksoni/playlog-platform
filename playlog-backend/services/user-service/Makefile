.PHONY: help start-dev stop-dev rebuild-dev logs-dev

ENV_FILE := .env
COMPOSE_DEV := docker compose -p playlog-user-service-dev --env-file $(ENV_FILE) -f ../../docker/pgadmin/compose.yaml -f ../../docker/postgres/user/compose.yaml

help:
	@echo "Available targets:"
	@echo "  make start-dev      - Start dev services"
	@echo "  make stop-dev       - Stop dev services"
	@echo "  make rebuild-dev    - Rebuild dev images (no cache)"
	@echo "  make logs-dev [svc] - Follow dev logs (optionally for a service)"

start-dev:
	@$(COMPOSE_DEV) up -d
	@$(MAKE) wait-dev

wait-dev:
	@echo "Waiting for user-service deps to be healthy..."
	@bash -c '\
		SERVICES=(postgres_users pgadmin); \
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
		echo "User service development environment is ready!"; \
		echo ""; \
		echo "Services:"; \
		echo "  - PostgreSQL:	localhost:5433"; \
		echo "  - pgAdmin:		http://localhost:5050"; \
		echo ""; \
		echo "View logs: make logs-dev [service_name]"; \
		echo "Stop: make stop-dev"; \
	'

stop-dev:
	@$(COMPOSE_DEV) down

rebuild-dev:
	@$(COMPOSE_DEV) build --no-cache

logs-dev:
	@$(COMPOSE_DEV) logs -f $(filter-out $@,$(MAKECMDGOALS))

%:
	@: