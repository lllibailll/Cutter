# Cutter
Cutter is a very basic url shortener.

It has been built using [Rust](https://www.rust-lang.org/), [Rocket.rs](https://rocket.rs/) and [Diesel.rs](https://diesel.rs/).

# Using Cutter

## Compiling
A Dockerfile is included to compile Cutter.
```
docker build -t cutter .
```
## Running

You need to provide an environmental variable named `ROCKET_DATABASES` with the database details following this format: `{Cutter={url="mysql://user:password@host/DB"}}`.

Example:
```
docker run \
--name cutter -d \
-p 10001:10001 \
-e ROCKET_PORT=10001 \
-e ROCKET_DATABASES='{Cutter={url="mysql://user:password@host/DB"}}' \
cutter:latest
```
