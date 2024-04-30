.PHONY: build test lint \
	style-check spell-check warnings-check \
	prettier

### Build ###

build: 
	cargo build


### Test ###
test:
	cargo test --workspace --exclude hyperdrive-math && \
	cargo test --package hyperdrive-math -- --test-threads=1

### Lint ###

lint:
	cargo check && cargo clippy && cargo fmt --check

spell-check:
	npx cspell crates/**/**/*.rs --gitignore