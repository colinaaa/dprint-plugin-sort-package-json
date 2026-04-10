# dprint-plugin-sort-package-json

Wasm module for `dprint-plugin-sort-package-json`.

## Install

```bash
npm install @dprint/formatter dprint-plugin-sort-package-json
```

## Usage

```js
import fs from "node:fs";
import { createFromBuffer } from "@dprint/formatter";
import { getPath } from "dprint-plugin-sort-package-json";

const formatter = createFromBuffer(fs.readFileSync(getPath()));

const formattedText = formatter.formatText({
  filePath: "package.json",
  fileText: '{"name":"test","version":"1.0.0"}',
});

console.log(formattedText);
```

## Links

- Repository: https://github.com/colinaaa/dprint-plugin-sort-package-json
- Issues: https://github.com/colinaaa/dprint-plugin-sort-package-json/issues
