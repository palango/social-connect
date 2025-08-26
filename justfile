# Build the project
build:
    cd rust && cargo build

# Build in release mode
build-release:
    cd rust && cargo build --release

# Run tests
test:
    cd rust && cargo test

# Format code
fmt:
    cd rust && cargo fmt --all

# Check formatting
fmt-check:
    cd rust && cargo fmt --all -- --check

# Run clippy linter
lint:
    cd rust && cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (format check, lint, test)
check: fmt-check lint test

# Clean build artifacts
clean:
    cd rust && cargo clean

# Show project info
info:
    @echo "Workspace members:"
    @cd rust && cargo metadata --format-version 1 | jq -r '.workspace_members[]'