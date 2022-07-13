# JWT Spring Authentication Server

SSO service based on JWT. 

More information about the available endpoints [here](openapi.yaml)

## Build and development

### Required software

- Docker 20.10+
- Java JDK 17+
- Maven 3.8+

### Creating a development environment

Run the following command to create all necessary services for AuthServ

```shell
docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d
```

```yaml
# A postgres instance stores all credentials
postgres:
  username: postgres
  password: password
  database: auth
  host: 0.0.0.0
  port: 5432
```

### Building and running the application

Use [Apache Maven Wrapper](https://maven.apache.org/wrapper/) for running and
building this application

### Environment variables

| Variable                | Description                                                 | Default value |
|-------------------------|-------------------------------------------------------------|---------------|
| JWT_PRIVATE_KEY         | Location of the RSA private key used to sign the JWT tokens |               |
| JWT_PUBLIC_KEY          | Location of the RSA public key used to verify JWT tokens    ||
| JWT_CLAIMS_ISSUER       | The issuer name (`iss` claim)                               |               |
| JWT_CLAIMS_EXPIRE       | TTL for each JWT token                                      ||
| JWT_CLAIMS_ROLESCLAIMS  | Claim name used to store all Spring roles                   | `roles`       |
| SPRING_PROFILES_DEFAULT | Spring profile used                                         | `dev`         |

> See more environment variables
>
on [Spring docs](https://docs.spring.io/spring-boot/docs/current/reference/html/application-properties.html)

## More information

Created using the following resources

- <https://github.com/Yoh0xFF/java-spring-security-example>
- <https://github.com/spring-projects/spring-security-samples/tree/main/servlet/spring-boot/java/jwt/login?rgh-link-date=2021-11-26T13%3A50%3A30Z>
- <https://docs.spring.io/spring-security/reference/servlet/oauth2/resource-server/jwt.html>
