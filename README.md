```shell
# https://stackoverflow.com/a/44474607/19176002
openssl genrsa -out keypair.pem 2048
openssl rsa -in keypair.pem -pubout -out public.key
openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in keypair.pem -out private.key
```
