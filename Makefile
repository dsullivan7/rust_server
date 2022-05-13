DOCKER_POSTGRES = postgres:12.7
DOCKER_ALPINE = alpine:3.13.5
DOCKER_RUST = rust:1-alpine3.15

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
	docker-compose up --build db-migrate

.PHONY: db-seed
db-seed:
	docker-compose up --build db-seed

.PHONY: db-init
db-init:
	docker-compose up --build db-init

.PHONY: run
run:
	go run ./cmd/app.go

.PHONY: run-docker
run-docker:
	docker-compose --verbose up --build run

.PHONY: build-docker
build-docker:
	docker-compose run rust cargo build --release

.PHONY: test-docker
test-docker:
	docker-compose run --rm golang go test -v $(TESTS)

.PHONY: docs
docs:
	redoc-cli build ./docs/main.yaml -o docs.html

.PHONY: lint
lint:
	golangci-lint run

.PHONY: lint-fix
lint-fix:
	golangci-lint run --fix

.PHONY: lint-docker
lint-docker:
	docker run --rm -v ${PWD}:/data -w /data ${DOCKER_GOLANG_LINT} golangci-lint run --timeout=3m
