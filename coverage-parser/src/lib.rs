mod attribution_structures;
mod coverage_structures;
mod types;

use crate::coverage_structures::CoverageReport;
use crate::types::Result;

pub struct CoverageParser;

impl CoverageParser {
    pub fn parse_file(path: &str) -> Result<CoverageReport> {
        let content = std::fs::read_to_string(path)?;
        let report: CoverageReport = serde_json::from_str(&content)?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let path = "../example.json".to_string();
        let parsed = CoverageParser::parse_file(&path).unwrap();
        assert!(&parsed.meta.show_contexts);
        println!("{:?}", parsed);
    }
}
