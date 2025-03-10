# Database and server configuration
DB_URL = postgres://postgres:postgres@localhost:5444/app_db
REDIS_URL = redis://127.0.0.1:6392
JWT_SECRET = secret
LOG_LEVEL = info

# Declare phony targets (those that don't represent files)
.PHONY: dev build run clean help

# Default target when just running 'make'
.DEFAULT_GOAL := help

dev:
	RUST_BACKTRACE=1 LOG_LEVEL=$(LOG_LEVEL) cargo watch -x "run --bin social_network -- --database-url $(DB_URL) --redis-url $(REDIS_URL) --jwt-secret $(JWT_SECRET)"

build:
	cargo build --release

run: build
	LOG_LEVEL=$(LOG_LEVEL) ./target/release/social_network --database-url $(DB_URL) --redis-url $(REDIS_URL) --jwt-secret $(JWT_SECRET)

clean:
	cargo clean

# Help command to display available targets
help:
	@echo "Available targets:"
	@echo "  dev    - Run the application with hot reloading for development"
	@echo "  build  - Build the release version of the application"
	@echo "  run    - Build (if needed) and run the release version"
	@echo "  clean  - Remove build artifacts"
	@echo "  help   - Display this help message"
