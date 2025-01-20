release: build
	wasm-bindgen --out-name xorshift --out-dir dist --target web target/wasm32-unknown-unknown/wasm-release/wireworld-xorshift-798.wasm

build:
	cargo build --profile wasm-release --target wasm32-unknown-unknown

