build:
	wasm-pack build --target web && cd www && yarn remove rust-wasm-test && yarn add rust-wasm-test@file:../pkg

dev:
	cd www && yarn dev
