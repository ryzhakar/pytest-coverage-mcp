# pytest-coverage-mcp User Story Book

## Context

This MCP server provides attribution data to language models writing tests, enabling behavior-focused test generation instead of coverage-percentage optimization.

## Agents

**Engineers**: Delegate test writing to language models, expect meaningful test outcomes  
**Language Models**: Generate tests using available tools and data sources  
**Provider**: Maintains attribution data and MCP server infrastructure

## Story Links

### Test Quality Domain

Engineers delegate test writing to language models because test writing requires understanding code behavior at scale.

Language models optimize for coverage percentage because it's the primary available testing metric.

Language models write superficial tests because coverage percentage misleads test quality assessment.

Engineers expect meaningful tests because superficial tests waste effort without validating behaviors.

Language models need behavior-validation signals because coverage percentage optimization produces meaningless tests.

Provider builds attribution resources because language models need behavior-validation signals.

### Attribution Access Domain

Language models access attribution resources because engineers delegate test quality concerns to them.

Language models query `attribution://of-test-elements-covering/{source_path}` when generating tests for specific behaviors.

Language models query `attribution://of-source-elements-covered-by/{test_path}` when validating test coverage scope.

Language models examine line-level execution patterns because test intent must align with actual execution.

Provider includes covered line numbers because language models need to verify behavior-targeting accuracy.

Provider uses pytest-format paths because language models need unambiguous behavior-to-test mapping.

### Data Freshness Domain

Language models require fresh attribution data because test-writing sessions are iterative.

Provider watches coverage file changes because language models require fresh attribution data.

Provider notifies language models when coverage data changes because test-writing sessions require fresh attribution data.

Language models re-query attribution resources when provider notifies about coverage changes.

### Behavioral Outcomes Domain

Engineers receive better tests because language models optimize for behavior validation instead of coverage percentage.

Language models write targeted tests because attribution data replaces misleading coverage metrics.

Provider constrains superficial optimization because language models need precise behavior-validation signals.

Engineers trust test outcomes because language models validate specific behaviors instead of executing arbitrary code paths.

## Resource Specifications

### Attribution Resources

**Pattern**: `attribution://of-test-elements-covering/{source_element_path}`  
**Response**: List of test paths with covered line numbers for the source element

**Pattern**: `attribution://of-source-elements-covered-by/{test_element_path}`  
**Response**: List of source elements with covered line numbers for the test

### Path Format

**Source Elements**: `module/file.py::ClassName::method_name` or `module/file.py::function_name` or `module/file.py::__module__`  
**Test Elements**: `tests/test_file.py::test_function_name`

### Notification Format

**Trigger**: Coverage file modification  
**Message**: Attribution data refreshed for `{coverage_file_path}` - query attribution resources for updated coverage information

## Usage Examples

### Generating Tests for Specific Behavior

**LLM Query**: `attribution://of-test-elements-covering/my_project/analytics.py::DataProcessor::validate_input`

**Response**:
```json
{
  "query": "attribution://of-test-elements-covering/my_project/analytics.py::DataProcessor::validate_input",
  "elements": [
    {
      "test_path": "tests/test_analytics.py::test_input_validation_happy_path",
      "covered_lines": [45, 47, 49],
      "attribution_strength": 0.6
    },
    {
      "test_path": "tests/test_analytics.py::test_input_validation_edge_cases", 
      "covered_lines": [45, 50, 52],
      "attribution_strength": 0.8
    }
  ]
}
```

**LLM Reasoning**: "The validation logic has partial coverage. I should write tests targeting lines 46, 48, 51 to cover the missing validation branches."

### Validating Test Coverage Scope

**LLM Query**: `attribution://of-source-elements-covered-by/tests/test_analytics.py::test_new_feature`

**Response**:
```json
{
  "query": "attribution://of-source-elements-covered-by/tests/test_analytics.py::test_new_feature",
  "elements": [
    {
      "source_path": "my_project/analytics.py::DataProcessor::validate_input",
      "covered_lines": [45, 46, 47],
      "element_type": "method"
    },
    {
      "source_path": "my_project/analytics.py::DataProcessor::process_data",
      "covered_lines": [72, 75, 78],
      "element_type": "method"
    }
  ]
}
```

**LLM Reasoning**: "This test actually exercises both validation and processing logic. The test name suggests it targets a specific feature, but it's covering broader functionality. I should split this into focused tests."

## Implementation Principles

### Atomic and Composable Stories

Each story represents a single causal link in the system behavior. Stories compose to describe complex workflows without redundancy.

### Semantic Resource Naming

Resource URIs encode the relationship direction and query intent, eliminating ambiguity for machine consumers.

### Behavior-First Attribution

Attribution data prioritizes understanding behavior validation over achieving coverage metrics.

### Real-time Feedback Loops

Coverage changes trigger immediate notification to enable iterative test development workflows.

---

*This story book serves as both specification and philosophy for attribution-driven test development.*
