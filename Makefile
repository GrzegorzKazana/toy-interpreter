build-all: build-interpreter build-frontend

build-interpreter:
	cd interpreter && \
	wasm-pack build --release --out-dir ../interpreter-js

build-frontend:
	cd frontend && \
	export NODE_ENV=production && \
	npm run export

build-frontend-ci:
	cd frontend && \
	export NODE_ENV=production && \
	export BASE_PATH=/toy-interpreter && \
	npm run export -- --basepath toy-interpreter

test-all: test-interpreter test-frontend

test-interpreter:
	cd interpreter && \
	cargo test

test-frontend:
	cd frontend && \
	npm run test

test-frontend-ci:
	cd frontend && \
	npm i && \
	npm run test