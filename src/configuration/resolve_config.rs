use dprint_core::configuration::*;

use super::Configuration;

pub fn resolve_config(
  config: ConfigKeyMap,
  _global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;

  let resolved_config = Configuration {
    sort_scripts: get_value(&mut config, "sortScripts", true, &mut diagnostics),
  };

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  }
}

#[cfg(test)]
mod tests {
  use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};

  use super::resolve_config;

  #[test]
  fn uses_true_by_default() {
    let result = resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
    assert!(result.diagnostics.is_empty());
    assert!(result.config.sort_scripts);
  }

  #[test]
  fn reads_sort_scripts_and_reports_unknown_properties() {
    let result = resolve_config(
      ConfigKeyMap::from([("sortScripts".to_string(), false.into()), ("notReal".to_string(), true.into())]),
      &GlobalConfiguration::default(),
    );

    assert!(!result.config.sort_scripts);
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(result.diagnostics[0].property_name, "notReal");
  }
}
