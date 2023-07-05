start:
	cargo run --release

test:
	ab -c 100 -n 10000 http://127.0.0.1:8080
