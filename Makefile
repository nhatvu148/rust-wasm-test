build-wasm:
	wasm-pack build --target web

dev:
	cd www && yarn dev
