# Design Document: Attribution-Driven Coverage MCP

## Problem Statement

**Core Issue**: Language models optimize for coverage percentage rather than meaningful test behavior validation, leading to "coverage theater" - superficial tests that inflate metrics without providing value.

**Root Cause**: Existing coverage tools provide aggregate metrics but lack precise attribution between specific tests and specific code elements.

## Design Principles

### 1. Attribution Over Aggregation
- **Traditional**: "87% coverage" (meaningless aggregate)
- **Our Approach**: "Line 47 covered by test_analytics_helper" (precise attribution)

### 2. Bidirectional Relationship Mapping
- **Test → Source**: What does this test actually cover?
- **Source → Test**: Which tests cover this function/method/class?

### 3. LLM-Optimized Interfaces
- **Semantic URIs**: Resource names encode query intent
- **Structured responses**: Machine-parseable attribution data
- **Real-time updates**: Fresh data for iterative test-writing sessions

## Architecture

### Multi-Crate Workspace Structure

```
pytest-coverage-mcp/
├── coverage-parser/     # Parse pytest coverage.json
├── attribution-engine/  # Build bidirectional mappings  
└── mcp-server/         # Expose MCP resources
```

**Rationale**: Clear separation of concerns enables independent testing, reuse, and potential future extraction of components.

### Resource URI Schema

```
attribution://of-test-elements-covering/{source_element_path}
attribution://of-source-elements-covered-by/{test_element_path}
```

**Design Decision**: Bidirectional resource types with explicit directionality in the URI itself prevents ambiguity for LLM consumers.

### Path Format Standards

**Source Elements**: `module/file.py::ClassName::method_name`
**Test Elements**: `tests/test_main.py::test_function_name`

**Rationale**: Hierarchical path format mirrors code structure while remaining unambiguous for both humans and machines.

## Data Flow

### 1. Coverage Generation
```bash
coverage run -m pytest tests
coverage json --show-contexts --pretty
```

### 2. Parse & Transform
```
coverage.json → CoverageReport → SourceElement[] + TestElement[]
```

### 3. Attribution Mapping
```
AttributionEngine::build_mappings(elements) → BiMap<Test, Source>
```

### 4. MCP Resource Exposure
```
attribution://... → AttributionResponse (JSON)
```

### 5. File Watching & Notifications
```
coverage.json change → notify MCP clients → re-query resources
```

## Key Data Structures

### Core Models
```rust
struct SourceElement {
    path: String,           // "my_project/main.py"
    element_path: String,   // "ComplexAnalytics::process_data"
    covered_lines: Vec<u32>,
}

struct TestElement {
    path: String,           // "tests.test_main.test_analytics_processing"
    normalized_path: String, // "tests/test_main.py::test_analytics_processing"
}
```

### Attribution Relationships
```rust
struct AttributionMap {
    test_to_source: HashMap<TestElement, Vec<SourceElement>>,
    source_to_test: HashMap<SourceElement, Vec<TestElement>>,
}
```

## Implementation Strategy

### Phase 1: Foundation (✅ COMPLETE)
- Workspace setup
- Coverage JSON parsing  
- Core data structures

### Phase 2: Attribution Engine (🔄 IN PROGRESS)
- Bidirectional mapping construction
- Element-level attribution extraction
- Context-to-element resolution

### Phase 3: MCP Server (⏭️ NEXT)
- Resource endpoint implementation
- Pattern matching for queries
- JSON response formatting

### Phase 4: Real-time Updates (⏭️ FUTURE)
- File watching with `notify` crate
- MCP notification dispatch
- Incremental update optimization

## Technical Decisions

### Why Rust?
- **Performance**: Fast parsing of large coverage files
- **Type Safety**: Prevent attribution mapping errors
- **Concurrency**: Efficient file watching and client handling
- **WebAssembly**: Potential future browser-based tooling

### Why Multi-Crate Workspace?
- **Modularity**: Each component testable in isolation
- **Reusability**: Coverage parser potentially useful standalone
- **Team Development**: Clear ownership boundaries

### Why MCP Protocol?
- **LLM Integration**: Native support in LLM development environments
- **Standardization**: Well-defined resource/tool protocol
- **Real-time**: Notification support for fresh data

## Risk Mitigation

### Coverage File Size
**Risk**: Large projects generate massive coverage.json files  
**Mitigation**: Streaming parser, lazy loading, incremental updates

### Path Format Variations  
**Risk**: Different pytest configurations produce different context formats  
**Mitigation**: Configurable path normalization, extensive test coverage

### LLM Query Patterns
**Risk**: Unexpected query patterns break resource assumptions  
**Mitigation**: Pattern matching with graceful degradation, comprehensive logging

## Success Metrics

### Engineering Outcomes
- Reduced time to write meaningful tests
- Increased confidence in test coverage attribution
- Elimination of coverage inflation incidents

### LLM Behavior Changes  
- Tests target specific behavior elements
- Reduced redundant test generation
- Improved test-to-requirement traceability

### System Performance
- Sub-100ms attribution query response times
- Real-time coverage update notifications
- Zero false positives in attribution mapping

## Future Evolution

### Potential Enhancements
- Branch coverage attribution (beyond line coverage)
- Historical attribution tracking
- Integration with other coverage tools (Go, Rust, etc.)
- Browser-based attribution visualization

### API Extensibility
- Plugin system for custom attribution rules
- Multiple coverage format support
- Distributed attribution for monorepos

---

*This design prioritizes clarity, precision, and machine-readability to enable a new paradigm of LLM-assisted test development.*
