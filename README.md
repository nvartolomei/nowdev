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
  --output linode-api
```