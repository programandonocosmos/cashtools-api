include .env.local

run:
	@DATABASE_URL=$(DATABASE_URL) SENDGRID_API_KEY=$(SENDGRID_API_KEY) EMAIL_FROM=$(EMAIL_FROM) cargo run

proxy:
	@flyctl proxy 5432 -a $(DB_APPNAME)

deploy:
	@flyctl deploy

migration-run:
	@DATABASE_URL=$(DATABASE_URL) diesel migration run