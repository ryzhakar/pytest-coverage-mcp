use coverage_parser;
use attribution_engine;

#[test]
fn builds_attribution_from_report() {
    let path = "../.example.json".to_string();
    let parsed = coverage_parser::CoverageParser::parse_file(&path).unwrap();
    let (modules, classes, funcs) = coverage_parser::CoverageParser::report_source_elements_from(&parsed);
    let attribution = attribution_engine::AttributionEngine::new(modules, classes, funcs);
    dbg!(attribution);
}

