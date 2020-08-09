# 🐜 http-debug
Http debugger service to test something (eg: reverse proxies)

## Install

You can install docker image using docker pull from [public repository](https://hub.docker.com/r/themisir/http-debug).

```sh
docker run -p 8080:8080 themisir/http-debug
```

Or you can use docker-compose:

```yml
services:
  http-debug:
    image: themisir/http-debug:latest
    ports:
      - "80:80"
    environment:
      - PORT=80
```

## Environment

| Variable    | Default | Description              |
|:-----------:|:-------:|:------------------------:|
| **PORT**    | 8080    | Http server port         |
| **VERBOSE** | FALSE   | Log requests to console  |

## Arguments

| Argument    | Short version | Description             |
|:-----------:|:-------------:|:-----------------------:|
| `--verbose` | `-v`          | Log requests to console |
