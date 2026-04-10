# dprint-plugin-sort-package-json

Sorts `package.json` files with dprint.

## Usage

Add the plugin to your `dprint.json`:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/colinaaa/dprint-plugin-sort-package-json/latest/plugin.wasm"
  ]
}
```

The plugin formats files named `package.json`.

## Configuration

```json
{
  "sortPackageJson": {
    "locked": true,
    "sortScripts": true
  }
}
```

- `locked`: Standard dprint plugin-config flag that prevents this config block from being overridden or extended.
- `sortScripts`: Sorts keys inside the `scripts` object. Default: `true`.
