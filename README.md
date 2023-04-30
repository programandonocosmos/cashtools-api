# CashTools API

This is the backend of a mobile app called CashTools. It exists to manage personal money. You can follow the app development on the [twitch channel](https://www.twitch.tv/programandonocosmos). Check the [mobile app repository](https://github.com/programandonocosmos/cashtools-app) too. It's written in Rust mainly using Rocket, Juniper, Diesel and Postgres.

## Dependencies

Unfortunately, you will need to install `libpq-dev` and `openssl-dev` manually to run the project. To do it run:

```bash
sudo apt install libpq-dev openssl-dev
```

## How to run locally + local database

Start a Postgres instance locally and add the database url in a file called `.env.local`. It should look like this:

```bash
DATABASE_URL=...
JWT_SECRET=...
ENV=DEV
API_PORT=8080
```

To run all migrations in your database, run:

```bash
make migration-run
```

To start the API simply run the following command:

```bash
make run
```

## How to run locally + remote database

To do it you should add a new variable to you `.env.local` called `DB_APPNAME` and set the `DATABASE_URL` correctly. You can get those informations from fly.io dashboard. 

After it, run:

```bash
make proxy
```

> *WARNING*: If it doesn't work, check if you have fly CLI installed and correctly authenticated.

Now you can just run the project as you expect:

```bash
make run
```