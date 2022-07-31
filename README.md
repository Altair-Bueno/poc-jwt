# Proof of Concept: Json Web Tokens

This repository contains a POC of how to use JWT with a SSO service to
authenticate a user across multiple services.

## Contents

- [authserv](authserv/)(Spring-Boot): SSO service based on basic authentication.
  Generates JWT tokens that other services consume
- [axum-transform](axum-transform/)(Rust): Offers text transformation utilities
  to authenticated users
- [spring-ping](spring-ping)(Spring-Boot): Simple ping server that only responds
  to authenticated users
- [tsdeno](tsdeno/)(Deno + Oak): Sample Oak application that manages Dinos!
- [slowapi](slowapi/)(Fast API): TODO crud
- [svelte-front](svelte-front/)(Svelte + TS): SPA that consumes most services

## Configuration details

### Generating a RSA key pair

Asymmetric cryptography is required to sign and verify JWT. Only the
authentication server (`Authserv`) requires access to the private key. Check
each project for more information about how you should provide the public key

```shell
# https://stackoverflow.com/a/44474607/19176002
openssl genrsa -out keypair.pem 2048
openssl rsa -in keypair.pem -pubout -out public.key
openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in keypair.pem -out private.key
```

## Example deployment: Docker compose

A sample Docker compose is provided. `svelte-front` will be available at
<http://localhost:8080>

```shell
# To start the service
docker compose up -d
# To stop the service
docker compose down
```

> Note: Docker compose requires building the docker containers from source
