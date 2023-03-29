format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet
run:
	cargo run 

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"command": "5"}' \
  		--output-format json \
  		random-num

all: format lint test run