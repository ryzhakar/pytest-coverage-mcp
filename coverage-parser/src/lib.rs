pub mod coverage_structures;
pub mod types;

use crate::coverage_structures::CoverageReport;
use crate::types::{ParseError, RawAttributionMap, Result};
use std::collections::HashMap;
pub struct CoverageParser;

impl CoverageParser {
    pub fn parse_file(path: &str) -> Result<CoverageReport> {
        let content = std::fs::read_to_string(path)?;
        let report: CoverageReport = serde_json::from_str(&content)?;

        Self::validate_report(&report)?;
        Ok(report)
    }

    fn validate_report(report: &CoverageReport) -> Result<()> {
        if !report.meta.show_contexts {
            return Err(ParseError::ContextDisabled);
        }
        // Unless the context string is empty,
        // at least one context must contain `test` as a prefix.
        let allowed_prefixes = ["test", "tests"];
        let has_at_least_one_test_context = report
            .files
            .values()
            // Flatten to context arrays
            .flat_map(|file| file.contexts.values())
            // and then to context strings
            .flat_map(|context_array| context_array.iter())
            // and then to prefix match statuses
            .flat_map(|context| {
                allowed_prefixes
                    .iter()
                    .map(|prefix| context.starts_with(prefix))
            })
            .any(|context| context);
        if !has_at_least_one_test_context {
            return Err(ParseError::WrongContextFormat);
        }
        Ok(())
    }

    // Report non-accumulated context mappings
    // of modules, classes and function-likes.
    pub fn report_source_elements_from(
        coverage_report: &CoverageReport,
    ) -> (RawAttributionMap, RawAttributionMap, RawAttributionMap) {
        let mut module_map: RawAttributionMap = HashMap::new();
        let mut class_map: RawAttributionMap = HashMap::new();
        let mut func_map: RawAttributionMap = HashMap::new();
        // Modules
        for (file_path, file_data) in &coverage_report.files {
            let mut file_context_map: HashMap<String, Vec<String>> = HashMap::new();
            for (module_line, module_tests) in &file_data.contexts {
                file_context_map.insert(module_line.to_owned(), module_tests.to_owned());
            }
            module_map.insert((file_path.clone(), "".to_string()), file_context_map);

            // Classes
            for (class_name, class_data) in &file_data.classes {
                let mut class_context_map: HashMap<String, Vec<String>> = HashMap::new();
                for (class_line, class_tests) in &class_data.contexts {
                    class_context_map.insert(class_line.to_owned(), class_tests.to_owned());
                }
                class_map.insert(
                    (file_path.clone(), class_name.to_owned()),
                    class_context_map,
                );
            }

            // Funcs
            for (func_name, func_data) in &file_data.functions {
                let mut func_context_map: HashMap<String, Vec<String>> = HashMap::new();
                for (func_line, func_tests) in &func_data.contexts {
                    func_context_map.insert(func_line.to_owned(), func_tests.to_owned());
                }
                func_map.insert((file_path.clone(), func_name.to_owned()), func_context_map);
            }
        }
        (module_map, class_map, func_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let path = "../.example.json".to_string();
        let parsed = CoverageParser::parse_file(&path).unwrap();
        let (modules, classes, funcs) = CoverageParser::report_source_elements_from(&parsed);
        assert!(&parsed.meta.show_contexts);
        assert!(!modules.is_empty());
        assert!(!classes.is_empty());
        assert!(!funcs.is_empty());
        println!("{:?}", parsed);
    }

    // #[test]
    // fn test_normalize_path() {
    //     // Normal case
    //     assert_eq!(
    //         CoverageParser::normalize_path_from("tests.test_main.test_analytics_processing"),
    //         "tests/test_main.py::test_analytics_processing"
    //     );
    //
    //     // Single part
    //     assert_eq!(
    //         CoverageParser::normalize_path_from("standalone_test"),
    //         "standalone_test"
    //     );
    //
    //     // Empty string
    //     assert_eq!(
    //         CoverageParser::normalize_path_from(""),
    //         ""
    //     );
    //
    //     // Two-part path
    //     assert_eq!(
    //         CoverageParser::normalize_path_from("tests.test_helper"),
    //         "tests.py::test_helper"
    //     );
    // }
}
