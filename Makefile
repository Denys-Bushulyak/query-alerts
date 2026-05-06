.PHONY: test run all clean

all: run

clean:
	cargo clean

test:
	set -a; source .env; set +a; cargo test

run: test
	set -a; source .env; set +a; cargo run
