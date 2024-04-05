# Justfile

# Build the project
build:
    cargo build --verbose

# Run tests
test:
    cargo test --verbose

# Lint the project
lint:
    cargo clippy -- -D warnings
    pre-commit run -a

# Run the project
run:
    cargo run

# Clean the project
clean:
    cargo clean

