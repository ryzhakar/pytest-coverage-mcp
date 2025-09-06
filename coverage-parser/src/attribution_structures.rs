use crate::types::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SourceElement {
    pub path: String,
    // "ClassName::method_name" or "function_name" or "__module__"
    pub element_path: String,
    pub covered_lines: LineNumberVector,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TestElement {
    // "tests.test_main.test_analytics_processing"
    pub path: String,
    // "tests/test_main.py::test_analytics_processing"
    pub normalized_path: String,
}
