const assert = require("node:assert/strict");

const { createFromBuffer } = require("@dprint/formatter");
const { getPath } = require("./index.js");

async function main() {
  const formatter = await createFromBuffer(await require("node:fs/promises").readFile(getPath()));

  const result = await formatter.formatText({
    filePath: "package.json",
    fileText: JSON.stringify(
      {
        scripts: {
          test: "vitest",
          build: "tsc"
        }
      },
      null,
      2
    ) + "\n",
    overrideConfig: {
      sortScripts: true
    }
  });

  assert.equal(
    result,
    '{\n  "scripts": {\n    "build": "tsc",\n    "test": "vitest"\n  }\n}\n'
  );
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
