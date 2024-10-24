### Initialize database

```
eval $(cat .env.dev) make db-start
eval $(cat .env.dev.docker) make db-init
eval $(cat .env.dev.docker) make db-migrate
eval $(cat .env.dev.docker) make db-seed
```

### Build and run binary

```
eval $(cat .env.dev) make build
./
```

### Run in development

```
eval $(cat .env.dev) RUST_LOG=debug cargo run
```

### Update cargo packages

```
cargo upgrade
```
