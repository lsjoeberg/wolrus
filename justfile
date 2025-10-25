# List recipes
default:
  just --list

# Format all files
fmt:
  cargo fmt --all

# Lint all targets
lint:
  cargo clippy --all-targets --all-features

# Build all targets with all features
build:
  cargo build --all-targets --all-features

# Run test
test:
  cargo test

# Run all tasks
all: fmt lint build test

