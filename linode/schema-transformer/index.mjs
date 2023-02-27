import fs from "fs";
import RefParser from "json-schema-ref-parser";

const inputSchema = await RefParser.dereference("../openapi.yaml");

await fs.promises.writeFile(
  "../transformed-schema.json",
  JSON.stringify(inputSchema, null, 2)
);

console.log("Done!");
