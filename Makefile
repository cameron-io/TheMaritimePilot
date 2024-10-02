SHELL := /bin/bash

SERVER_NAME := maritime_pilot
BUILD_TAG := latest

.PHONY: dev
dev: build
	docker compose up -d server

.PHONY: build
build: $(SERVER_SOURCE)
	docker build -t $(SERVER_NAME):$(BUILD_TAG) .

.PHONY: down
down:
	docker compose down

.PHONY: clean
clean:
	rm -rf target
