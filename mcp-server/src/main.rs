use attribution_engine;
use coverage_parser;

fn main() {
    let path = ".example.json".to_string();
    let py_test_dir = "tests".to_string();
    let parsed = coverage_parser::CoverageParser::parse_file(&path, &py_test_dir).unwrap();
    let (modules, classes, funcs) = coverage_parser::CoverageParser::report_source_elements_from(&parsed);
    let engine = attribution_engine::AttributionEngine::new(modules, classes, funcs, &py_test_dir);
    dbg!(engine.full_accumulated_attribution);
}
