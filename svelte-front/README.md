# Svelte front

Provides a simple SPA application that showcases all microservices

## Build and development

### Required software

- Node.js 16+

### Creating a development environment

Run the following command to create mock servers for each required service

```sh
docker compose up -d
```

### Building and running the application

```shell
# Install dependencies
npm install
# Generate API client
npm run api
# Build the application
npm run build
# Run the development server
npm run dev
# Run Prettier
npm run format
```

### Environment variables

| Variable             | Description                                                   | Default value |
| -------------------- | ------------------------------------------------------------- | ------------- |
| VITE_AUTHSERV_URL    | Location of AuthServ                                          |               |
| VITE_SPRINGPING_URL  | Location of Springping                                        |               |
| VITE_CREDENTIALS_KEY | Key used to store the credentials object inside Local Storage | `credentials` |
