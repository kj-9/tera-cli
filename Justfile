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


# commit and tag prerelease: dry-run
pre-release-dry-run:
    cz bump --prerelease alpha --changelog --dry-run

# commit and tag prerelease
pre-release:
    cz bump --prerelease alpha --changelog

release-dry-run:
    cz bump --changelog --dry-run

release:
    cz bump --changelog
