# Docker Compose Commands for Different Environments

# Development - with hot reload and dev tools
dev:
	docker compose -f docker-compose.dev.yml up --build

dev-detached:
	docker compose -f docker-compose.dev.yml up --build -d

# Production - optimized builds
prod:
	docker compose -f docker-compose.prod.yml up --build

prod-detached:
	docker compose -f docker-compose.prod.yml up --build -d

# Testing - run tests in containers
test:
	docker compose -f docker-compose.test.yml up --build --abort-on-container-exit

# Database only - just PostgreSQL
db:
	docker compose -f docker-compose.db-only.yml up -d

# Clean up
clean:
	docker compose -f docker-compose.dev.yml down -v
	docker compose -f docker-compose.prod.yml down -v
	docker compose -f docker-compose.test.yml down -v
	docker compose -f docker-compose.db-only.yml down -v

# Stop all services
stop:
	docker compose -f docker-compose.dev.yml down
	docker compose -f docker-compose.prod.yml down
	docker compose -f docker-compose.test.yml down
	docker compose -f docker-compose.db-only.yml down

# View logs
logs-dev:
	docker compose -f docker-compose.dev.yml logs -f

logs-prod:
	docker compose -f docker-compose.prod.yml logs -f

# Shell access
shell-backend-dev:
	docker compose -f docker-compose.dev.yml exec backend bash

shell-backend-prod:
	docker compose -f docker-compose.prod.yml exec backend bash

# Database access
db-shell:
	docker compose -f docker-compose.db-only.yml exec postgres psql -U guild_user -d guild_genesis
