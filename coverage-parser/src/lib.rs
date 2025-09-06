mod attribution_structures;
mod coverage_structures;
mod types;

use crate::attribution_structures::{SourceElement, TestElement};
use crate::coverage_structures::{CoverageData, CoverageReport};
use crate::types::{ParseError, Result};
use std::collections::{HashMap, HashSet};

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
        let allowed_prefixes = vec!["test", "tests"];
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

    pub fn extract_source_elements(report: &CoverageReport) -> Vec<SourceElement> {
        let mut elements = Vec::<SourceElement>::new();
        for (file_path, file_data) in &report.files {
            // Module-level element
            elements.push(SourceElement {
                path: file_path.clone(),
                element_path: "__module__".to_string(),
                covered_lines: file_data.executed_lines.clone(),
            });

            // Functions
            for (func_name, func_data) in &file_data.functions {
                elements.push(SourceElement {
                    path: file_path.clone(),
                    element_path: func_name.clone(),
                    covered_lines: func_data.executed_lines.clone(),
                });
            }

            // Classes and methods
            Self::extract_class_elements_from(file_path, &file_data.classes, &mut elements, None);
        }
        elements
    }

    fn extract_class_elements_from(
        file_path: &str,
        classes: &HashMap<String, CoverageData>,
        element_container: &mut Vec<SourceElement>,
        parent_node: Option<&str>,
    ) {
        for (name, class_data) in classes {
            let full_class_path = match parent_node {
                Some(parent_path) => format!("{}::{}", parent_path, name),
                None => name.clone(),
            };

            // The class itself
            element_container.push(SourceElement {
                path: file_path.to_string(),
                element_path: full_class_path.clone(),
                covered_lines: class_data.executed_lines.clone(),
            });

            // Nested classes
            Self::extract_class_elements_from(
                file_path,
                &class_data.classes,
                element_container,
                Some(&full_class_path),
            );

            // Class methods

            for (method_name, method_data) in &class_data.functions {
                element_container.push(SourceElement {
                    path: file_path.to_string(),
                    element_path: format!("{}::{}", full_class_path, method_name),
                    covered_lines: method_data.executed_lines.clone(),
                });
            }
        }
    }

    pub fn extract_test_elements(report: &CoverageReport) -> Vec<TestElement> {
        let mut test_contexts = HashSet::<String>::new();
        for file_data in report.files.values() {
            Self::collect_contexts_recursive(file_data, &mut test_contexts);
        }

        test_contexts
            .into_iter()
            .filter(|ctx| !ctx.is_empty())
            .map(|ctx| TestElement {
                path: ctx.clone(),
                normalized_path: Self::normalize_path_from(&ctx),
            })
            .collect()
    }

    fn collect_contexts_recursive(data: &CoverageData, coverage_contexts: &mut HashSet<String>) {
        for test_list in data.contexts.values() {
            for test in test_list {
                coverage_contexts.insert(test.clone());
            }
        }

        for func_data in data.functions.values() {
            Self::collect_contexts_recursive(func_data, coverage_contexts);
        }

        for class_data in data.classes.values() {
            Self::collect_contexts_recursive(class_data, coverage_contexts);
        }
    }

    fn normalize_path_from(coverage_py_formatted: &str) -> String {
        if coverage_py_formatted.is_empty() {
            return coverage_py_formatted.to_string();
        }
        // Convert "tests.test_main.test_analytics_processing"
        // to "tests/test_main.py::test_analytics_processing"
        // to maintain consistency with pytest.
        let parts: Vec<&str> = coverage_py_formatted.split('.').collect();
        // Module level or malformed?
        if parts.len() < 2 {
            return coverage_py_formatted.to_string();
        }
        let (module_parts, test_function_name) = parts.split_at(parts.len() - 1);
        let pymodule_path = module_parts.join("/");
        format!("{}.py::{}", pymodule_path, test_function_name[0],)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let path = "../.example.json".to_string();
        let parsed = CoverageParser::parse_file(&path).unwrap();
        let source_elements = CoverageParser::extract_source_elements(&parsed);
        let test_elements = CoverageParser::extract_test_elements(&parsed);
        assert!(&parsed.meta.show_contexts);
        assert!(!source_elements.is_empty());
        assert!(!test_elements.is_empty());
        println!("{:?}", parsed);
    }

    #[test]
    fn test_normalize_path() {
        // Normal case
        assert_eq!(
            CoverageParser::normalize_path_from("tests.test_main.test_analytics_processing"),
            "tests/test_main.py::test_analytics_processing"
        );

        // Single part
        assert_eq!(
            CoverageParser::normalize_path_from("standalone_test"),
            "standalone_test"
        );

        // Empty string
        assert_eq!(
            CoverageParser::normalize_path_from(""),
            ""
        );

        // Two-part path
        assert_eq!(
            CoverageParser::normalize_path_from("tests.test_helper"),
            "tests.py::test_helper"
        );
    }

}
