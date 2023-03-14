import fs from "fs";
import RefParser from "json-schema-ref-parser";

const inputSchema = await RefParser.parse("../openapi.yaml");

inputSchema.components.schemas.LinodeType.properties.addons.properties.backups.properties.price.properties.hourly.type =
  "number";
inputSchema.components.schemas.LinodeType.properties.addons.properties.backups.properties.price.properties.monthly.type =
  "number";

inputSchema.components.schemas.LinodeType.properties.price.properties.hourly.type =
  "number";
inputSchema.components.schemas.LinodeType.properties.price.properties.monthly.type =
  "number";
delete inputSchema.components.schemas.Linode.properties.host_uuid.format;

for (const path of Object.keys(inputSchema.paths)) {
  const isInstancesPath = path.startsWith("/linode/instances");
  const isTypesPath = path.startsWith("/linode/types");

  if (!isInstancesPath && !isTypesPath) {
    delete inputSchema.paths[path];
  }
}

const dereferencedSchema = await RefParser.dereference(inputSchema);

await fs.promises.writeFile(
  "../transformed-schema.json",
  JSON.stringify(dereferencedSchema, null, 2)
);

console.log("Done!");
