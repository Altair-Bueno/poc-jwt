FROM denoland/deno:alpine-1.24.0
RUN apk update
RUN apk add make sqlite
COPY . /application
WORKDIR /application
RUN make cache
RUN mkdir -p /oak
ENV OAK_META_DIR=/oak
CMD make run
