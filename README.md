```
eval $(cat .env.dev) make db-init
eval $(cat .env.dev.docker) make db-init
eval $(cat .env.dev.docker) make db-migrate
eval $(cat .env.dev) make build
```

### Update cargo packages

```
cargo upgrade
```
