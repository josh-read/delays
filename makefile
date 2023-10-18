lib:
	cargo build --lib --release

app:
	trunk build --release --dist docs --public-url ./delays/ --features app-deps

serve:
	trunk serve --open --dist docs --features app-deps