#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceElementType {
    Module,
    Class,
    FunctionLike,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceElement {
    pub original_file_path: String,
    pub original_element_path: String,
    pub normalized_full_path: String,
    pub element_type: SourceElementType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TestElement {
    // "tests.test_main.test_analytics_processing"
    pub original_full_path: String,
    // "tests/test_main.py::test_analytics_processing"
    pub normalized_full_path: String,
}

impl SourceElement {
    pub fn from_parts(
        raw_file_path: &str,
        raw_element_path: &str,
        element_type_override: Option<SourceElementType>,
    ) -> Self {
        Self {
            original_file_path: raw_file_path.to_string(),
            original_element_path: raw_element_path.to_string(),
            normalized_full_path: Self::normalized_from(raw_file_path, raw_element_path),
            element_type: Self::element_type_from(raw_element_path),
        }
    }
    fn normalized_from(raw_file_path: &str, raw_element_path: &str) -> String {
        let element_path = if raw_element_path.is_empty() {
            "__module__"
        } else {
            raw_element_path
        };
        format!("{}::{}", raw_file_path, element_path)
    }
    fn element_type_from(raw_element_path: &str) -> SourceElementType {
        let element_parts: Vec<&str> = raw_element_path.split('.').collect();
        match element_parts[element_parts.len() - 1].chars().nth(0) {
            Some(ch) => {
                if ch.is_uppercase() {
                    SourceElementType::Class
                } else {
                    SourceElementType::FunctionLike
                }
            }
            // Empty string
            None => SourceElementType::Module,
        }
    }
}

impl TestElement {
    pub fn from_parts(raw_test_path: &str) -> Self {
        Self {
            original_full_path: raw_test_path.to_string(),
            normalized_full_path: Self::normalize_path_from(raw_test_path),
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
    fn test_normalize_path() {
        // Normal case
        assert_eq!(
            TestElement::normalize_path_from("tests.test_main.test_analytics_processing"),
            "tests/test_main.py::test_analytics_processing"
        );

        // Single part
        assert_eq!(
            TestElement::normalize_path_from("standalone_test"),
            "standalone_test"
        );

        // Empty string
        assert_eq!(
            TestElement::normalize_path_from(""),
            ""
        );

        // Two-part path
        assert_eq!(
            TestElement::normalize_path_from("tests.test_helper"),
            "tests.py::test_helper"
        );
    }
}
