# dprint-plugin-sort-package-json

Sorts `package.json` files with dprint.

## Usage

Add the npm package to your `dprint.json`:

```json
{
  "plugins": [
    "npm:dprint-plugin-sort-package-json@0.1.0"
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
