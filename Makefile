build-wasm:
	cd interpreter && \
	wasm-pack build --release --out-dir ../interpreter-js
