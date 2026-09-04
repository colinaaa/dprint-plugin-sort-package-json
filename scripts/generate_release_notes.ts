import { generateChangeLog } from "https://raw.githubusercontent.com/dprint/automation/0.9.0/changelog.ts";

const version = Deno.args[0];
const changelog = await generateChangeLog({
  versionTo: version,
});
const text = `## Changes

${changelog}

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

### npm (recommended)

Add the npm package to your project's dprint configuration file:

\`\`\`jsonc
{
  "sortPackageJson": {
    // sortPackageJson config goes here
  },
  "plugins": [
    "npm:dprint-plugin-sort-package-json@${version}"
  ]
}
\`\`\`

### GitHub

Alternatively, use the plugin from GitHub Releases:

\`\`\`jsonc
{
  "sortPackageJson": {
    // sortPackageJson config goes here
  },
  "plugins": [
    "https://github.com/colinaaa/dprint-plugin-sort-package-json/releases/download/${version}/plugin.wasm"
  ]
}
\`\`\`

## JS Formatting API

* [npm package](https://www.npmjs.com/package/dprint-plugin-sort-package-json)
`;

console.log(text);
