use std::path::Path;

use anyhow::Result;
use anyhow::Context;
use sort_package_json::SortOptions;

use crate::configuration::Configuration;

pub type FormatRange = Option<std::ops::Range<usize>>;

pub fn format_text(file_path: &Path, file_text: &str, config: &Configuration) -> Result<Option<String>> {
  format_text_with_range(file_path, file_text, None, config)
}

pub fn format_text_with_range(
  _file_path: &Path,
  file_text: &str,
  range: FormatRange,
  config: &Configuration,
) -> Result<Option<String>> {
  if range.is_some() {
    return Ok(None);
  }

  let sorted_text = sort_package_json::sort_package_json_with_options(
    file_text,
    &SortOptions {
      pretty: true,
      sort_scripts: config.sort_scripts,
    },
  )
  .context("Failed sorting package.json with sort-package-json")?;

  if sorted_text == file_text {
    Ok(None)
  } else {
    Ok(Some(sorted_text))
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::{format_text, format_text_with_range};
  use crate::configuration::Configuration;

  #[test]
  fn returns_none_when_input_is_already_sorted() {
    let file_text = concat!(
      "{\n",
      "  \"name\": \"pkg\",\n",
      "  \"scripts\": {\n",
      "    \"build\": \"tsc\",\n",
      "    \"test\": \"vitest\"\n",
      "  }\n",
      "}\n",
    );

    let result = format_text(Path::new("package.json"), file_text, &Configuration { sort_scripts: true }).unwrap();

    assert_eq!(result, None);
  }

  #[test]
  fn sorts_scripts_when_enabled() {
    let file_text = concat!(
      "{\n",
      "  \"scripts\": {\n",
      "    \"test\": \"vitest\",\n",
      "    \"build\": \"tsc\"\n",
      "  }\n",
      "}\n",
    );

    let result = format_text(Path::new("package.json"), file_text, &Configuration { sort_scripts: true }).unwrap();

    assert_eq!(
      result,
      Some(
        concat!(
          "{\n",
          "  \"scripts\": {\n",
          "    \"build\": \"tsc\",\n",
          "    \"test\": \"vitest\"\n",
          "  }\n",
          "}\n",
        )
        .to_string()
      )
    );
  }

  #[test]
  fn preserves_scripts_order_when_disabled() {
    let file_text = concat!(
      "{\n",
      "  \"scripts\": {\n",
      "    \"test\": \"vitest\",\n",
      "    \"build\": \"tsc\"\n",
      "  }\n",
      "}\n",
    );

    let result = format_text(Path::new("package.json"), file_text, &Configuration { sort_scripts: false }).unwrap();

    assert_eq!(result, None);
  }

  #[test]
  fn preserves_utf8_bom() {
    let file_text = concat!(
      "\u{feff}",
      "{\n",
      "  \"scripts\": {\n",
      "    \"test\": \"vitest\",\n",
      "    \"build\": \"tsc\"\n",
      "  }\n",
      "}\n",
    );

    let result = format_text(Path::new("package.json"), file_text, &Configuration { sort_scripts: true }).unwrap();

    assert_eq!(
      result,
      Some(
        concat!(
          "\u{feff}",
          "{\n",
          "  \"scripts\": {\n",
          "    \"build\": \"tsc\",\n",
          "    \"test\": \"vitest\"\n",
          "  }\n",
          "}\n",
        )
        .to_string()
      )
    );
  }

  #[test]
  fn normalizes_mixed_newlines_to_lf() {
    let file_text = "{\r\n  \"name\": \"pkg\"\n}\r\n";

    let result = format_text(Path::new("package.json"), file_text, &Configuration { sort_scripts: true }).unwrap();

    assert_eq!(
      result,
      Some(
        concat!(
          "{\n",
          "  \"name\": \"pkg\"\n",
          "}\n",
        )
        .to_string()
      )
    );
  }

  #[test]
  fn normalizes_consistent_crlf_input_to_lf() {
    let file_text = concat!(
      "{\r\n",
      "  \"version\": \"1.0.0\",\r\n",
      "  \"name\": \"pkg\"\r\n",
      "}\r\n",
    );

    let result = format_text(Path::new("package.json"), file_text, &Configuration { sort_scripts: true }).unwrap();

    assert_eq!(
      result,
      Some(
        concat!(
          "{\n",
          "  \"name\": \"pkg\",\n",
          "  \"version\": \"1.0.0\"\n",
          "}\n",
        )
        .to_string()
      )
    );
  }

  #[test]
  fn returns_none_for_range_format_requests() {
    let file_text = concat!(
      "{\n",
      "  \"scripts\": {\n",
      "    \"test\": \"vitest\",\n",
      "    \"build\": \"tsc\"\n",
      "  }\n",
      "}\n",
    );

    let result = format_text_with_range(
      Path::new("package.json"),
      file_text,
      Some(std::ops::Range { start: 0, end: file_text.len() }),
      &Configuration { sort_scripts: true },
    )
    .unwrap();

    assert_eq!(result, None);
  }

  #[test]
  fn returns_error_for_invalid_json() {
    let result = format_text(Path::new("package.json"), "{", &Configuration { sort_scripts: true });

    let err = result.unwrap_err();
    assert_eq!(
      err.to_string(),
      "Failed sorting package.json with sort-package-json"
    );
  }
}
