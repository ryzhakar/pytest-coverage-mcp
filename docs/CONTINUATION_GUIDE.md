# Implementation Continuation Guide

## Development Context

**Note**: This implementation represents human-written code developed through LLM-guided learning. The architecture, design decisions, and code structure are the result of human engineering work, with an LLM providing mentorship, explanations, and technical guidance throughout the development process. This collaborative approach demonstrates effective human-AI learning partnerships in software development.

## Current State Summary

**✅ COMPLETED:**
- Workspace structure with 3 crates defined
- `coverage-parser` crate fully implemented with:
  - JSON parsing for pytest coverage format
  - Data structure modeling (CoverageReport, FileData, etc.)
  - Element extraction (SourceElement, TestElement)
  - Test path normalization (`tests.test_main.test_func` → `tests/test_main.py::test_func`)

**🔄 NEXT IMMEDIATE STEP:**
Create the `attribution-engine` crate that builds bidirectional mappings.

## Step-by-Step Continuation

### 1. Create Attribution Engine Crate

```bash
# From workspace root
cargo new --lib attribution-engine
```

Add to `attribution-engine/Cargo.toml`:
```toml
[package]
name = "attribution-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
coverage-parser = { path = "../coverage-parser" }
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
thiserror.workspace = true
```

### 2. Implement Core Attribution Logic

Create `attribution-engine/src/lib.rs`:

```rust
use coverage_parser::{CoverageReport, SourceElement, TestElement};
use std::collections::HashMap;

pub struct AttributionEngine {
    test_to_source: HashMap<TestElement, Vec<SourceElement>>,
    source_to_test: HashMap<SourceElement, Vec<TestElement>>,
}

impl AttributionEngine {
    pub fn build_from_report(report: &CoverageReport) -> Self {
        let source_elements = coverage_parser::CoverageParser::extract_source_elements(report);
        let test_elements = coverage_parser::CoverageParser::extract_test_elements(report);
        
        // TODO: Build bidirectional mapping by analyzing contexts
        // For each line in source_elements.covered_lines:
        //   1. Find which tests caused that line execution (from contexts)
        //   2. Build test_to_source mapping
        //   3. Build source_to_test mapping (inverse)
        
        Self {
            test_to_source: HashMap::new(),
            source_to_test: HashMap::new(),
        }
    }
    
    pub fn get_tests_covering_source(&self, source_path: &str) -> Vec<&TestElement> {
        // Implementation needed
        vec![]
    }
    
    pub fn get_sources_covered_by_test(&self, test_path: &str) -> Vec<&SourceElement> {
        // Implementation needed  
        vec![]
    }
}
```

**Key Implementation Challenge**: You need to analyze the `contexts` field in the coverage JSON to build the bidirectional mapping. Each line number maps to test contexts that caused its execution.

### 3. Create MCP Server Crate

```bash
cargo new --bin mcp-server
```

Add to `mcp-server/Cargo.toml`:
```toml
[package]
name = "mcp-server"
version = "0.1.0"
edition = "2021"

[dependencies]
coverage-parser = { path = "../coverage-parser" }
attribution-engine = { path = "../attribution-engine" }
rmcp = "0.6.3"
tokio = { version = "1.0", features = ["full"] }
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
notify = "6.0"  # For file watching
glob = "0.3"    # For pattern matching
```

### 4. Implement MCP Resource Handlers

Key resources to implement:
- `attribution://of-test-elements-covering/{source_element_path}`
- `attribution://of-source-elements-covered-by/{test_element_path}`

**Pattern matching support**: Handle paths like `attribution://of-test-elements-covering/my_project/*`

### 5. Add File Watching

Implement coverage.json file watching using the `notify` crate to send MCP notifications when coverage data changes.

## Critical Implementation Notes

### Context-to-Attribution Mapping

The most complex part is analyzing the `contexts` field:

```json
{
  "contexts": {
    "15": ["tests.test_main.test_analytics_helper"],
    "16": ["tests.test_main.test_analytics_helper", "tests.test_main.test_other"]
  }
}
```

This means:
- Line 15 was executed by `test_analytics_helper`
- Line 16 was executed by both `test_analytics_helper` AND `test_other`

Your attribution engine must:
1. Parse these contexts for each source element
2. Map test contexts back to TestElement instances
3. Build bidirectional relationships

### URI Pattern Matching

Support these query patterns:
- Exact: `attribution://of-test-elements-covering/my_project/main.py::ComplexAnalytics::process_data`
- Wildcard: `attribution://of-test-elements-covering/my_project/*`
- Test-specific: `attribution://of-source-elements-covered-by/tests/test_main.py::test_analytics_processing`

### Response Format

Standardize JSON responses:
```json
{
  "query": "attribution://of-test-elements-covering/my_project/main.py::ComplexAnalytics",
  "elements": [
    {
      "test_path": "tests/test_main.py::test_analytics_processing",
      "covered_lines": [6, 8, 10, 11, 18, 20],
      "attribution_strength": 0.8
    }
  ]
}
```

## Testing Strategy

### Unit Tests
- Test attribution mapping with sample coverage.json
- Verify bidirectional relationship consistency
- Test URI pattern matching logic

### Integration Tests
- End-to-end MCP resource queries
- File watching and notification delivery
- Performance with large coverage files

### Sample Data
Use the provided coverage.json example for initial testing, then generate additional test cases with different complexity levels.

## Immediate Next Actions

1. **Run existing tests**: `cd coverage-parser && cargo test`
2. **Create attribution-engine**: Implement the bidirectional mapping logic
3. **Build MCP server skeleton**: Basic resource routing
4. **Test with sample data**: Use the provided coverage.json

## Success Criteria

**Attribution Engine Ready**: Can query both directions with sample coverage data  
**MCP Server Functional**: Responds to basic attribution queries  
**File Watching Active**: Detects coverage.json changes and notifies  

The foundation is solid - the next implementer has clear data structures and a proven parsing strategy. The remaining work is primarily about building the relationship mappings and exposing them via MCP resources.
