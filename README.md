This is a Rust workspace with two crates:

```
prewave/
├── app/              # CLI application binary
│   └── src/main.rs   # Entry point
├── lib/              # Core library
│   ├── src/
│   │   ├── query.rs           # Core query/matching logic
│   │   ├── dtos/              # API response parsing
│   │   ├── entities/          # Domain entities
│   │   └── data_providers/    # API data fetching
│   ├── benches/               # Criterion benchmarks
│   └── tests/                 # Integration tests
├── Cargo.toml        # Workspace definition
└── Makefile          # Build/run/test commands
```

## Prerequisites

- Rust toolchain (Edition 2024 or later)
- Cargo package manager

## Setup

1. Clone the repository:

```bash
git clone <repository-url>
cd prewave
```

2. Configure environment variables:

```bash
cp .env.sample .env
```

3. Edit `.env` with your API credentials:

```
API_KEY=username:secret
API_ENTRYPOINT=https://services.prewave.ai/adminInterface/api
DEBUG=false
```

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `API_KEY` | API credentials in format `username:secret` | `john_doe:secret` |
| `API_ENTRYPOINT` | API base URL | `https://services.prewave.ai/adminInterface/api` |
| `DEBUG` | Enable debug mode (`true`/`false`) | `false` |

When `DEBUG=true`, API responses are saved to `alerts.json` and `terms.json`.

## Usage

```bash
# Run the application
make
# or
cargo run

# Run with debug mode
DEBUG=true cargo run

# Run tests
make test
# or
cargo test

# Run benchmarks to compare 2 algorithms performance
make bench
# or
cargo bench

# Generate code coverage report using llvm-cov
make cov
# or
cargo llvm-cov 

# Run demo with regex algorithm (release mode, pretty-printed JSON output)
make demo_regex

# Run demo with case-insensitive matching algorithm (release mode, pretty-printed JSON output)
make demo_simple

# Clean build artifacts
make clean
# or
cargo clean
```

## Output

The application outputs results as JSON mapping term IDs to matching alert IDs:

```json
{
  "term_id_1": ["alert_id_1", "alert_id_2"],
  "term_id_2": ["alert_id_3"]
}
```

## Architecture

- **DTO-to-Entity Conversion** - Clean separation between data transfer objects (API response parsing) and domain entities with validation
- **Query Engine** - Groups query terms by language, builds case-insensitive regex patterns, and matches them against alert content text

## License

Apache License 2.0
