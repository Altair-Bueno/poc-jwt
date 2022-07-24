# TS Deno

More information about the available endpoints [here](openapi.yml)

## Build and development

### Required software

- Deno
- GNU Make

### Building and running the application

```sh
# Download all dependencies
make cache
# Run the application
make run
```

### Configuration

Use a `json` file to configure the server. By default, the server looks for a file
called `oak.json` located on the root directory. The 

```jsonc
// The configuration file cannot contain comments
{
  // Server port
  "port": 9100,
  // Server hostname
  "hostname": "0.0.0.0",
  // File where the public key used to validate the token is located
  "publicKey": null,
  // Algorithm used to sign the JWTs
  "algorithm": "RS256"
}
``` 

> See more configuration options [here](https://doc.deno.land/https://deno.land/x/oak@v10.6.0/mod.ts/~/ListenOptions)

