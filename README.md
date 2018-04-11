# Hyper Microservice

Just a pet project, experimenting with various libs to get a microservice up and running with little boilerplate and as many real-world features as possible.

## Dependencies
You need the diesel CLI crate:
`cargo install --no-default-features --features postgres`

## Usage
Build with `cargo build`.

Start a dockerized postgres server:
`docker run -p 5432:5432 --name hyper-microservice-db -d --restart unless-stopped postgres`

Run diesel setup

Run with `cargo run`

## Endpoints
    GET /
    Says hello to the world.

    GET /urls/<value>
    Looks up a stored url.
    Example: curl 'http://localhost:3000/urls/foo' 

    POST /urls?short=<short>&long=<long>
    Stores the long url under the short key.
    Example: curl -X POST 'http://localhost:3000/urls/short=foo&long=www.google.com'
