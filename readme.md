# Scrobbler backend

An API for things.

## Running

```
docker-compose run --rm --service-ports scrobbler
```

You can also add something like `cargo run --bin api` if you just want to run the API.

## Watching and running

```
docker-compose run --rm --service-ports scrobbler-dev
```

## Running migrations

This project is using [Diesel](http://diesel.rs/) as the ORM.

I didn't want to install Postgres on my system so I could use the Diesel CLI, so it's dockerized.

```
docker-compose run --rm diesel-cli
# cd core
# diesel setup
# diesel migration run
```

## Building for ARM

As of Docker v19.03.13, `docker buildx` can only be used by enabling experimental features.

```
docker buildx build --platform linux/arm/v7 .
```
