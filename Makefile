all: wasm-thumbnail-py/src/wasm_thumbnail/data/wasm_thumbnail.wasm

wasm-thumbnail-py/src/wasm_thumbnail/data/wasm_thumbnail.wasm: wasm-thumbnail/src/lib.rs
	cd wasm-thumbnail && cargo build --release
	cp wasm-thumbnail/target/wasm32-unknown-unknown/release/wasm_thumbnail.wasm wasm-thumbnail-py/src/wasm_thumbnail/data/
	cp wasm-thumbnail/target/wasm32-unknown-unknown/release/wasm_thumbnail.wasm wasm-thumbnail-rb/lib/wasm/thumbnail/rb/data/
