<div align="center">
  <img src="https://github.com/dappface/www.dappface.com/raw/master/static/icon-128x128.png" alt="DAPPFACE Logo" />

  <h1>DAPPFACE Rust API</h1>

  <p>
    <a href="https://github.com/dappface/api-rust/actions?workflow=Deploy">
      <img src="https://github.com/dappface/api-rust/workflows/Deploy/badge.svg" />
    </a>
  </p>
</div>

## Start Locally

```
APP_ENV=${APP_ENV} SLACK_API_TOKEN=${SLACK_API_TOKEN} cargo run
```

## Start Docker Container

```
docker build -t dappface-rust-api .
docker run --init --rm -p 8080:8080 -e APP_ENV=${APP_ENV} -e SLACK_API_TOKEN=${SLACK_API_TOKEN} dappface-rust-api
```
