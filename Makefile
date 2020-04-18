build-all: build-interpreter build-frontend

build-interpreter:
	cd interpreter && \
	wasm-pack build --release --out-dir ../interpreter-js

build-frontend:
	cd frontend && \
	export NODE_ENV=production && \
	npm run export

test-interpreter:
	cd interpreter && \
	cargo test

test-frontend:
	cd frontend && \
	npm run test