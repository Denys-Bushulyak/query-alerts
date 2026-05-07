ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: test run all clean cov

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

cov:
	@echo "--- Running coverage analysis ---"
	cargo llvm-cov

demo_regex:
	@echo "--- Running demo ---"
	DEBUG=false cargo run --release -- --regex | json_pp

demo_simple:
	@echo "--- Running demo ---"
	DEBUG=false cargo run --release -- --simple | json_pp

bench:
	@echo "--- Running benchmarks ---"
	cargo bench
