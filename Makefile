DOCKER_POSTGRES = postgres:12.7
DOCKER_ALPINE = alpine:3.20.0
DOCKER_RUST = rustdocker/rustfmt_clippy:stable

ENVFILE ?= .env

TESTS ?= ./test/...

.PHONY: db-stop
db-stop:
	docker-compose -f docker-compose.yml stop postgres

.PHONY: db-start
db-start:
	docker-compose -f docker-compose.yml up -d postgres

.PHONY: db-remove
db-remove:
	docker-compose -f docker-compose.yml rm -s -v postgres && docker volume rm go_server_dbdata_12.7

.PHONY: db-migrate
db-migrate:
	docker compose up --build db-migrate

.PHONY: db-seed
db-seed:
	docker-compose up --build db-seed

.PHONY: db-init
db-init:
	docker compose up --build db-init

.PHONY: run
run:
	cargo run

.PHONY: run-docker
run-docker:
	docker-compose up --build run

.PHONY: build-docker
build-docker:
	docker-compose up --build build

.PHONY: test-docker
test-docker:
	docker-compose up --build test

.PHONY: docs
docs:
	redoc-cli build ./docs/main.yaml -o docs.html

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint-docker
lint-docker:
	docker compose up --build ci
	docker compose run ci cargo clippy --all-targets --all-features -- -D warnings
