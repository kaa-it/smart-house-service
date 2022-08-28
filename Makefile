.PHONY: watch clippy fmt_check fmt db_up db_down

watch:
	cargo watch -x 'check --color=always' -x 'test -- --color=always' -x run

clippy:
	cargo clippy --all --all-features --tests -- -D warnings

fmt_check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

test:
	cargo test --test api

db_up:
	docker-compose up -d

db_down:
	docker-compose down