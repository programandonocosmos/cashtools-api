include .env.local

run:
	@API_PORT=$(API_PORT) DATABASE_URL=$(DATABASE_URL) JWT_SECRET=$(JWT_SECRET) ENV=$(ENV) cargo run

run-debug:
	@RUST_LOG=debug DATABASE_URL=$(DATABASE_URL) JWT_SECRET=$(JWT_SECRET) ENV=$(ENV) cargo run

test:
	@RUST_LOG=debug DATABASE_URL=$(DATABASE_URL) cargo test

proxy:
	@flyctl proxy 5432 -a $(DB_APPNAME)

deploy:
	@flyctl deploy

migration-run:
	@DATABASE_URL=$(DATABASE_URL) diesel migration run

migration-revert:
	@DATABASE_URL=$(DATABASE_URL) diesel migration revert