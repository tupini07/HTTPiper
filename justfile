default:
	echo 'Hello, world!'

watch:
	cargo watch -x check -x test

clippy:
	cargo clippy

fix:
	cargo fix
