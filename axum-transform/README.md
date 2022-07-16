# Axum Transform

Sample microservice for JWT authorized users, written in Rust

More information about the available endpoints [here](openapi.yml)

## Build and development

### Required software

- Rust 1.62+

### Building and running the application

```shell
# NOTE: Add the `--release` flag to build release artifacts
# Build and run
cargo run
# Build
cargo build
```

### Configuration

You can configure the server modifying `axum.toml` or by setting environment
variables with the `AXUM_` prefix

| Key           | Description                                              | Default value |
| ------------- | -------------------------------------------------------- | ------------- |
| hostname      | Server IP address                                        | `127.0.0.1`   |
| port          | Server port                                              | `9100`        |
| jwt.publickey | Location of the RSA public key used to verify JWT tokens |               |
| jwt.algorithm | Algorithm used to verify the JWT signature               | `RS256`       |

> Note: Use `_` instead of `.` as a separator for environment variables (eg:
> `AXUM_JWT_PUBLICKEY='foo'`)
