FROM denoland/deno:alpine-1.24.0
RUN apk update
RUN apk add make
COPY . /application
WORKDIR /application
RUN make cache
CMD make run
