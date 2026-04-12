.PHONY: dev prod down logs

dev:
	docker compose --profile dev --profile prod down --remove-orphans
	docker compose --profile dev up -d

prod:
	docker compose --profile dev --profile prod down --remove-orphans
	docker compose --profile prod up --build

down:
	docker compose --profile dev --profile prod down --remove-orphans

logs:
	docker compose logs -f
