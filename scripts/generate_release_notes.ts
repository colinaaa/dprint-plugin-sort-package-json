import { generateChangeLog } from "https://raw.githubusercontent.com/dprint/automation/0.9.0/changelog.ts";

const version = Deno.args[0];
const changelog = await generateChangeLog({
  versionTo: version,
});
const text = `## Changes

${changelog}

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's dprint configuration file:

1. Specify the plugin url in the \`"plugins"\` array.
2. Add a \`"sortPackageJson"\` configuration property if desired.
   \`\`\`jsonc
   {
     // ...etc...
     "sortPackageJson": {
       // sortPackageJson config goes here
     },
     "plugins": [
       "https://plugins.dprint.dev/colinaaa/dprint-plugin-sort-package-json/${version}/plugin.wasm"
     ]
   }
   \`\`\`

## JS Formatting API

* [npm package](https://www.npmjs.com/package/dprint-plugin-sort-package-json)
`;

console.log(text);
