# Fast API TODO crud

Fast API JWT micro service for managing todos

More information about the available endpoints [here](openapi.yaml)

## Build and development

### Required software

- Poetry
- Python 3.10

### Building and running the application

Run the Uvicorn server by running the following command

```sh
# Add the `--refesh` flag to redeploy application on changes
poetry run uvicorn slowapi:app --port=9300
```

### Environment variables

| Variable      | Description                                                                            | Default value |
| ------------- | -------------------------------------------------------------------------------------- | ------------- |
| DATABASE_URL  | Database url. See [tortoise docs](https://tortoise.github.io) for compatible databases |               |
| JWT_ALGORITHM | Algorithm used to sign the JWT tokens                                                  | RS256         |
| JWT_PUBLICKEY | Location of the public key file                                                        |               |

> `.env` files can be used to load environment variables. See
> [pydantic docs](https://pydantic-docs.helpmanual.io)
