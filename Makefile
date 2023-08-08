SHELL := /bin/bash

SERVER_NAME := maritime_pilot
BUILD_TAG := latest

.PHONY: run
run: build
	sudo docker compose up -d server

.PHONY: build
build: $(SERVER_SOURCE) deps
	sudo docker build -t $(SERVER_NAME):$(BUILD_TAG) .

.PHONY: down
down:
	sudo docker compose down

.PHONY: admin
admin:
	sudo docker compose up -d admin
