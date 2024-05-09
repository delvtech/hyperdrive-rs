.PHONY: build test lint spell-check

### Build ###
build: 
	cargo build

### Test ###
test:
	cargo test

### Lint ###
lint:
	cargo check && cargo clippy && cargo fmt --check

### Spell-Check ###
spell-check:
	npx cspell crates/**/**/*.rs --gitignore