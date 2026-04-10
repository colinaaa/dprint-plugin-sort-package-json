use dprint_core::plugins::{FileMatchingInfo, PluginInfo};

pub const CONFIG_KEY: &str = "sortPackageJson";

pub fn file_matching_info() -> FileMatchingInfo {
  FileMatchingInfo {
    file_extensions: vec![],
    file_names: vec!["package.json".to_string()],
  }
}

pub fn plugin_info() -> PluginInfo {
  let version = env!("CARGO_PKG_VERSION");

  PluginInfo {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: version.to_string(),
    config_key: CONFIG_KEY.to_string(),
    help_url: "https://github.com/colinaaa/dprint-plugin-sort-package-json".to_string(),
    config_schema_url: format!(
      "https://plugins.dprint.dev/colinaaa/dprint-plugin-sort-package-json/{version}/schema.json"
    ),
    update_url: None,
  }
}

#[cfg(test)]
mod tests {
  use super::{CONFIG_KEY, file_matching_info, plugin_info};

  #[test]
  fn only_matches_package_json_by_file_name() {
    let file_matching_info = file_matching_info();

    assert!(file_matching_info.file_extensions.is_empty());
    assert_eq!(file_matching_info.file_names, vec!["package.json"]);
  }

  #[test]
  fn exposes_the_expected_config_key() {
    assert_eq!(CONFIG_KEY, "sortPackageJson");
  }

  #[test]
  fn exposes_colinaaa_repository_urls() {
    let info = plugin_info();
    let version = env!("CARGO_PKG_VERSION");

    assert_eq!(
      info.help_url,
      "https://github.com/colinaaa/dprint-plugin-sort-package-json"
    );
    assert_eq!(
      info.config_schema_url,
      format!(
        "https://plugins.dprint.dev/colinaaa/dprint-plugin-sort-package-json/{version}/schema.json"
      )
    );
  }
}
