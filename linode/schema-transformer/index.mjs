import fs from "fs";
import RefParser from "json-schema-ref-parser";

const inputSchema = await RefParser.parse("../openapi.yaml");

// These are not real UUIDs. At least according to the UUID library we use.
delete inputSchema.components.schemas.Linode.properties.host_uuid.format;

for (const path of Object.keys(inputSchema.paths)) {
  const isInstancesPath = path.startsWith("/linode/instances");
  const isTypesPath = path.startsWith("/linode/types");
  const isVolumesPath = path.startsWith("/volumes");

  if (!isInstancesPath && !isTypesPath && !isVolumesPath) {
    delete inputSchema.paths[path];
  }
}

const dereferencedSchema = await RefParser.dereference(inputSchema);

await fs.promises.writeFile(
  "../transformed-schema.json",
  JSON.stringify(dereferencedSchema, null, 2)
);

console.log("Done!");
