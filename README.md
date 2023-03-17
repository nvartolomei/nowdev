## User guide

Add to your `~/.ssh/config` `Include config_nowdev`

```
Usage: nowdev <COMMAND>

Commands:
  start  start a development environment
  stop   stop a development environment
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Development

```sh
brew install openapi-generator
```

```sh
pushd linode/schema-transformer
npm ci
node index.mjs
popd
```

```sh
openapi-generator generate --generator-name rust \
  --package-name linode-api \
  --input-spec ./linode/transformed-schema.json \
  --global-property apis,apiDocs=false,models,modelDocs=false,supportingFiles \
  --output linode-api

cargo fmt --package linode-api && cargo clippy --fix --package linode-api --allow-staged --allow-dirty
```
