# Makefile for tui-music-player
# Usage: make <target>   (run `make help` to list targets)

CARGO      := cargo
APP        := tui
CONFIG     := $(HOME)/.config/tui-music-player/config

.DEFAULT_GOAL := run
.PHONY: help build release run run-release check clippy fmt fmt-check test clean reset-config

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

build: ## Build all crates (debug)
	$(CARGO) build --workspace

release: ## Build all crates (optimized)
	$(CARGO) build --workspace --release

run: ## Build and run the TUI (debug)
	$(CARGO) run -p $(APP)

run-release: ## Build and run the TUI (optimized)
	$(CARGO) run -p $(APP) --release

check: ## Type-check without building binaries
	$(CARGO) check --workspace

clippy: ## Run the linter
	$(CARGO) clippy --workspace -- -D warnings

fmt: ## Format all code
	$(CARGO) fmt --all

fmt-check: ## Check formatting without changing files
	$(CARGO) fmt --all -- --check

test: ## Run all tests
	$(CARGO) test --workspace

clean: ## Remove build artifacts
	$(CARGO) clean

reset-config: ## Delete the saved login so the app asks again
	rm -f $(CONFIG)
	@echo "Removed $(CONFIG)"