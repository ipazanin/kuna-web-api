######################################################
#                       Basic
######################################################
build::
	cargo build

check::
	cargo check
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

test::
	 cargo test

format::
	cargo clippy --fix
	cargo fmt --all

clean::
	rm -rf ./target

######################################################
#                       Docker
######################################################
docker-build::
	docker build -t kuna-web-api:latest -f Dockerfile .

docker-run::
	docker run -p 4000:4000 kuna-web-api:latest

compose::
	docker compose -f compose.yml up  --build