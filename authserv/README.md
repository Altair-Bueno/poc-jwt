# JWT Spring Authentication Server

Created based on the following resources:

- https://github.com/Yoh0xFF/java-spring-security-example
- https://github.com/spring-projects/spring-security-samples/tree/main/servlet/spring-boot/java/jwt/login?rgh-link-date=2021-11-26T13%3A50%3A30Z
- https://docs.spring.io/spring-security/reference/servlet/oauth2/resource-server/jwt.html

# Generate a public/private RSA keypair for JWT

```shell
# https://stackoverflow.com/a/44474607/19176002
openssl genrsa -out keypair.pem 2048
openssl rsa -in keypair.pem -pubout -out public.key
openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in keypair.pem -out private.key
```

# Start the Postgres database server

```shell
docker compose -f docker-compose-dev.yml up -d
```
