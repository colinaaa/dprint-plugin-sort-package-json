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

## Credits

Thanks to the `sort-package-json` project, the Rust `sort_package_json` crate, and `dprint` for the ideas and foundations that made this plugin possible.

- [`sort-package-json`](https://github.com/keithamus/sort-package-json)
- [`sort_package_json`](https://github.com/oxc-project/sort-package-json)
- [`dprint`](https://github.com/dprint/dprint)
