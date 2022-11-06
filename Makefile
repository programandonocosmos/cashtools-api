include .env.local

run:
	@DATABASE_URL=$(DATABASE_URL) JWT_SECRET=$(JWT_SECRET) cargo run

proxy:
	@flyctl proxy 5432 -a $(DB_APPNAME)

deploy:
	@flyctl deploy

migration-run:
	@DATABASE_URL=$(DATABASE_URL) diesel migration run

migration-revert:
	@DATABASE_URL=$(DATABASE_URL) diesel migration revert