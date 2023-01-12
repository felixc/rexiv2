# SPDX-FileCopyrightText: 2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
# SPDX-License-Identifier: GPL-3.0-or-later


MSRV=1.63  # Minimum supported Rust version


help:
	@echo "Usage: make TARGET [-- ARGS...]\n"
	@echo "Available targets:"
	@awk -F ' +##' 'NF>1 {printf "\033[36m  %-18s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo

setup:  ## Install all tools for local development
	rustup toolchain add stable
	rustup toolchain add nightly
	rustup toolchain add $(MSRV)
	rustup component add llvm-tools-preview clippy rustfmt
	cargo install cargo-audit cargo-outdated rustfilt grcov

  ##

format:  ## Run auto-formatter
	cargo +nightly fmt

check:  ## Run linter/analysis tool
	cargo clippy --all-targets --all-features -- -D warnings

test:  ## Run all tests (or specify a test name/selector to run just that)
	cargo test --all-features -- $(filter-out $@, $(MAKECMDGOALS))

coverage:  ## Produce a test coverage report
	cargo +nightly clean
	RUSTDOCFLAGS="-C instrument-coverage -Z unstable-options --persist-doctests target/debug/doctestbins" \
		RUSTFLAGS="-C instrument-coverage" \
		LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw" \
		cargo +nightly test --all-features
	grcov ./target/coverage --binary-path ./target/debug/ --source-dir . \
		--ignore-not-existing --ignore "*examples*" --branch \
		--output-type html --output-path ./target/coverage/html/
	@printf "\nCoverage report available at: file:///$$(pwd)/target/coverage/html/index.html\n"

doc:  ## Generate documentation
	cargo clean
	cargo doc --no-deps --all-features
	@printf "\nDocs available at: file:///$$(pwd)/target/doc/rexiv2/index.html\n"

  ##

clean:  ## Remove all build artefacts
	cargo clean

release:  ## Run checks before releasing a new version
	rustup update
	cargo install $$(cargo install --list | awk '/    / {all=all$$0} END {print all}')
	cargo update
	cargo clean && cargo build --all-features && cargo test --all-features
	cargo clean && cargo +$(MSRV) build --all-features && cargo +$(MSRV) test --all-features
	cargo clean && cargo +nightly build --all-features && cargo +nightly test --all-features
	cargo outdated --root-deps-only
	cargo audit


.PHONY: help setup format check test coverage doc clean release
