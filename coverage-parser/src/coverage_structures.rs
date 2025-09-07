use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CoverageReport {
    pub meta: Meta,
    pub files: HashMap<String, CoverageData>,
    pub totals: CoverageSummary,
}

impl CoverageReport {
    pub fn as_validated(self, test_dir_name: &str) -> Result<Self> {
        if !self.meta.show_contexts {
            return Err(ParseError::ContextDisabled);
        }
        let allowed_prefixes = [test_dir_name];
        let has_at_least_one_test_context = 
            self.files.values()
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
        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub format: u32,
    pub version: String,
    pub timestamp: String,
    pub branch_coverage: bool,
    pub show_contexts: bool,
}

#[derive(Debug, Deserialize)]
pub struct CoverageData {
    #[serde(default)]
    pub executed_lines: LineNumberVector,
    #[serde(default)]
    pub missing_lines: LineNumberVector,
    #[serde(default)]
    pub excluded_lines: LineNumberVector,
    pub summary: CoverageSummary,
    pub contexts: HashMap<String, Vec<String>>,
    //
    pub executed_branches: Vec<BranchExit>,
    pub missing_branches: Vec<BranchExit>,
    //
    #[serde(default)]
    pub functions: HashMap<String, CoverageData>,
    #[serde(default)]
    pub classes: HashMap<String, CoverageData>,
}

#[derive(Debug, Deserialize)]
pub struct CoverageSummary {
    pub covered_lines: u32,
    pub num_statements: u32,
    //
    pub percent_covered: f64,
    pub percent_covered_display: String,
    //
    pub missing_lines: u32,
    pub excluded_lines: u32,
    //
    pub num_branches: u32,
    pub num_partial_branches: u32,
    pub covered_branches: u32,
    pub missing_branches: u32,
}
