ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: test run all clean

# Default target when just running 'make'
all: run

clean:
	cargo clean

test:
	@echo "--- Running tests ---"
	cargo test

run:
	@echo "--- Running application ---"
	cargo run

demo:
	@echo "--- Running demo ---"
	DEBUG=false cargo run --release | json_pp
