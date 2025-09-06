use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CoverageReport {
    pub meta: Meta,
    pub files: HashMap<String, CoverageData>,
    pub totals: CoverageSummary,
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
