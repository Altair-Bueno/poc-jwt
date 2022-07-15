# Spring Ping

Provides ping responses for authorized users. 

More information about the available endpoints [here](openapi.yml)

## Build and development

### Required software

- Java JDK 17+
- Maven 3.8+

### Building and running the application

Use [Apache Maven Wrapper](https://maven.apache.org/wrapper/) for running and
building this application

### Environment variables

| Variable                | Description                                                 | Default value |
|-------------------------|-------------------------------------------------------------|---------------|
| JWT_PUBLIC_KEY          | Location of the RSA public key used to verify JWT tokens    ||
| JWT_CLAIMS_ROLESCLAIMS  | Claim name used to store all Spring roles                   | `roles`       |
| SPRING_PROFILES_DEFAULT | Spring profile used                                         | `dev`         |

> See more environment variables
> on [Spring docs](https://docs.spring.io/spring-boot/docs/current/reference/html/application-properties.html)
