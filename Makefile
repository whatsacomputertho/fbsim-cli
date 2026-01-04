# Also supports
# - x86_64-pc-windows-gnu
# - x86_64-apple-darwin
# - x86_64-unknown-linux-gnu
# - aarch64-pc-windows-msvc
# - aarch64-apple-darwin
# - aarch64-unknown-linux-gnu
PLATFORM ?= x86_64-pc-windows-msvc
RELEASE_UPLOAD_URL ?=
BUILD_ARGS ?= --release --target=$(PLATFORM)
LINT_ARGS ?= --all-targets --all-features -- -D warnings

build-dependencies:
	rustup target add $(PLATFORM)

build:
	cargo build $(BUILD_ARGS)

lint:
	cargo clippy $(LINT_ARGS)

release:
	@bash ci/release.sh "$(RELEASE_UPLOAD_URL)" "$(PLATFORM)"

sec-dependencies:
	cargo install cargo-audit

sec:
	cargo audit
