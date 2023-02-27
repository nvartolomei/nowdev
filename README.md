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
  --input-spec ./linode/transformed-schema.json \
  --output linode-api
```