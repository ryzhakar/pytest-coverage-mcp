# Technical Reference & Implementation Status

## Workspace Structure

```
pytest-coverage-mcp/
├── Cargo.toml                    # Workspace definition
├── coverage-parser/              # ✅ IMPLEMENTED
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs               # Coverage JSON parsing & data modeling
│   └── tests/
├── attribution-engine/           # 🔄 NEXT STEP  
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs               # Bidirectional attribution mapping
│   └── tests/
└── mcp-server/                   # ⏭️ FUTURE
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs              # MCP server binary
    │   ├── resources.rs         # Attribution resource handlers  
    │   └── notifications.rs     # File watching & notifications
    └── tests/
```

## Completed Implementation

### Workspace Cargo.toml

```toml
[workspace]
members = [
    "coverage-parser",
    "attribution-engine", 
    "mcp-server"
]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
```

### Coverage Parser Implementation

**File**: `coverage-parser/src/lib.rs`

**Key Data Structures**:
```rust
// Raw JSON parsing structures
#[derive(Debug, Deserialize)]
pub struct CoverageReport {
    pub meta: Meta,
    pub files: HashMap<String, FileData>,
    pub totals: Summary,
}

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub executed_lines: Vec<u32>,
    pub missing_lines: Vec<u32>,
    pub excluded_lines: Vec<u32>,
    pub summary: Summary,
    pub contexts: HashMap<String, Vec<String>>, // line_num -> test_contexts
    pub executed_branches: Vec<[i32; 2]>,
    pub missing_branches: Vec<[i32; 2]>,
    pub functions: HashMap<String, FileData>,
    pub classes: HashMap<String, FileData>,
}

// Processed attribution structures  
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SourceElement {
    pub path: String,           // "my_project/main.py"
    pub element_path: String,   // "ClassName::method_name" 
    pub covered_lines: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TestElement {
    pub path: String,           // "tests.test_main.test_analytics_processing"
    pub normalized_path: String, // "tests/test_main.py::test_analytics_processing"
}
```

**Key Functions**:
- `CoverageParser::parse_file(path: &str) -> Result<CoverageReport>`
- `CoverageParser::extract_source_elements(report: &CoverageReport) -> Vec<SourceElement>`
- `CoverageParser::extract_test_elements(report: &CoverageReport) -> Vec<TestElement>`

**Test Path Normalization**:
```rust
// Converts "tests.test_main.test_analytics_processing" 
// to "tests/test_main.py::test_analytics_processing"
fn normalize_test_path(pytest_path: &str) -> String
```

## Expected Input Format

### Coverage Generation Commands

```bash
# Generate coverage with contexts
coverage run -m pytest tests
coverage json --show-contexts --pretty
```

### Required pyproject.toml Configuration

```toml
[tool.coverage.run]
branch = true
dynamic_context = "test_function"  # Critical for attribution
concurrency = ["greenlet", "thread"]

[tool.coverage.report]
fail_under = 85.0
precision = 2
skip_covered = true
sort = "Cover"
```

### Sample Coverage JSON Schema

**Key Structure**:
```json
{
  "meta": {
    "format": 3,
    "version": "8.0.0", 
    "show_contexts": true
  },
  "files": {
    "my_project/main.py": {
      "executed_lines": [1, 2, 5, 6, 8, 10],
      "contexts": {
        "1": [""],
        "2": ["tests.test_main.test_top_level"],
        "5": ["tests.test_main.test_analytics_processing"]
      },
      "functions": {
        "top_level_func": {
          "executed_lines": [2],
          "contexts": {"2": ["tests.test_main.test_top_level"]}
        }
      },
      "classes": {
        "ComplexAnalytics": {
          "executed_lines": [6, 8, 10],
          "functions": {
            "process_data": {
              "executed_lines": [8, 10], 
              "contexts": {
                "8": ["tests.test_main.test_analytics_processing"],
                "10": ["tests.test_main.test_analytics_processing"]
              }
            }
          }
        }
      }
    }
  }
}
```

## Next Implementation Phase

### Attribution Engine Interface

**File**: `attribution-engine/src/lib.rs` (TO BE IMPLEMENTED)

```rust
pub struct AttributionEngine {
    test_to_source: HashMap<TestElement, Vec<SourceElement>>,
    source_to_test: HashMap<SourceElement, Vec<TestElement>>,
}

impl AttributionEngine {
    pub fn build_from_report(report: &CoverageReport) -> Self;
    pub fn get_tests_covering_source(&self, source_path: &str) -> Vec<&TestElement>;
    pub fn get_sources_covered_by_test(&self, test_path: &str) -> Vec<&SourceElement>;
}
```

**Critical Implementation Task**: Analyze `contexts` field to build bidirectional mapping:

```rust
// For each file's contexts field:
// "contexts": {"15": ["tests.test_main.test_analytics_helper"]}
// 
// Need to map:
// TestElement("tests.test_main.test_analytics_helper") -> 
//   SourceElement("my_project/main.py::function_name", covered_lines: [15])
```

## MCP Resource Specification

### Resource URI Patterns

```
attribution://of-test-elements-covering/{source_element_path}
attribution://of-source-elements-covered-by/{test_element_path}
```

**Pattern Matching Support**:
- Exact: `attribution://of-test-elements-covering/my_project/main.py::ComplexAnalytics::process_data`
- Wildcard: `attribution://of-test-elements-covering/my_project/*`
- Module: `attribution://of-test-elements-covering/my_project/main.py::__module__`

### Response JSON Schema

```json
{
  "query": "attribution://of-test-elements-covering/{path}",
  "elements": [
    {
      "test_path": "tests/test_main.py::test_analytics_processing",
      "covered_lines": [6, 8, 10, 11, 18, 20],
      "attribution_strength": 0.8,
      "context_source": "pytest_contexts"
    }
  ]
}
```

### MCP Notification Format

```json
{
  "method": "notifications/resources/updated", 
  "params": {
    "uri": "attribution://",
    "message": "Attribution data refreshed for coverage.json - query attribution resources for updated coverage information"
  }
}
```

## Dependencies

### Core Dependencies
```toml
rmcp = "0.6.3"           # MCP server framework
serde = "1.0"            # JSON serialization
serde_json = "1.0"       # JSON parsing
tokio = "1.0"            # Async runtime
anyhow = "1.0"           # Error handling
thiserror = "1.0"        # Error types
notify = "6.0"           # File watching
glob = "0.3"             # Pattern matching
```

### Development Dependencies
```toml
[dev-dependencies]
tempfile = "3.0"         # Test file creation
tokio-test = "0.4"       # Async testing utilities
```

## Testing Strategy

### Unit Tests (Implemented)
- Path normalization (`test_normalize_test_path`)
- JSON parsing with sample coverage data
- Element extraction accuracy

### Integration Tests (Planned)
- End-to-end attribution mapping
- MCP resource query/response cycles
- File watching notification delivery

### Performance Tests (Future)
- Large coverage file parsing
- Attribution query response times
- Memory usage with complex hierarchies

## Key Implementation Notes

### Path Hierarchy Handling

**Nested Classes**: `ComplexAnalytics::Helpers::utility_method`
**Function Scope**: Module-level functions vs class methods
**Special Elements**: `__module__` for module-level coverage

### Context Analysis Complexity

**Multiple Test Coverage**: Single line covered by multiple tests
**Empty Contexts**: Module-level execution before test functions
**Hierarchical Contexts**: Method calls within class contexts

### Error Handling Strategy

**Parse Errors**: Graceful degradation with partial data
**Path Resolution**: Fallback strategies for malformed paths  
**Attribution Conflicts**: Resolution strategies for ambiguous mappings

---

*This reference provides the complete technical foundation for continuing implementation or onboarding team members.*
