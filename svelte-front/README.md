# Svelte front

Provides a simple SPA application that showcases all microservices

## Build and development

### Required software

- Node.js 16+

### Building and running the application

```shell
# Install dependencies
npm install
# Generate API client
npm run api
# Build the application
npm run build
```

### Environment variables

| Variable             | Description                                                   | Default value |
| -------------------- | ------------------------------------------------------------- | ------------- |
| VITE_AUTHSERV_URL    | Location of AuthServ                                          |               |
| VITE_SPRINGPING_URL  | Location of Springping                                        |               |
| VITE_CREDENTIALS_KEY | Key used to store the credentials object inside Local Storage | `credentials` |
