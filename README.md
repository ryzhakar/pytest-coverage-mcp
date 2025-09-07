# pytest-coverage-mcp

**Attribution-driven test coverage analysis for LLM-assisted development**

## Vision

Transform how language models write tests by providing precise, bidirectional coverage attribution data instead of misleading aggregate coverage percentages. This prevents "coverage theater" - the anti-pattern where tests are optimized for coverage metrics rather than meaningful behavior validation.

## Development Approach

**Human-Driven Implementation with LLM Mentorship**: This codebase represents human engineering work guided by language model mentorship. The code is written by a human developer, with an LLM serving as an interactive learning mentor - helping understand concepts, suggesting architectural approaches, and providing technical guidance. This collaborative learning approach demonstrates how LLMs can enhance human learning and development workflows without replacing human creativity and decision-making.

## Core Problem

Current test-writing workflows suffer from:
- **Coverage inflation**: LLMs optimize for percentage metrics instead of behavior validation
- **Attribution opacity**: No way to verify which specific tests cover which code elements
- **Feedback loop failure**: Developers can't validate that new tests actually cover intended behaviors

## Solution Architecture

**Multi-crate Rust workspace providing:**

1. **coverage-parser**: Parses pytest coverage.json into structured attribution data
2. **attribution-engine**: Builds bidirectional mappings between tests and source elements  
3. **mcp-server**: MCP server exposing attribution resources to LLM clients

**Resource-based attribution queries:**
```
attribution://of-test-elements-covering/{source_element_path}
attribution://of-source-elements-covered-by/{test_element_path}
```

## Key Design Decisions

- **Attribution-focused URIs**: Semantic resource names that encode query intent
- **Bidirectional mapping**: Test→Source and Source→Test relationships
- **Element-level granularity**: Function, class, and method-level attribution
- **Real-time updates**: File watching with MCP notifications
- **LLM-optimized interfaces**: Structured data designed for machine reasoning

## Current Implementation Status

✅ **Workspace structure defined**  
✅ **Core data models implemented**  
✅ **Coverage JSON parser completed**  
🔄 **Attribution engine** (next: bidirectional mapping)  
⏭️ **MCP server** (awaiting attribution engine)  
⏭️ **File watching & notifications**  

## Quick Start

```bash
git clone <repository>
cd pytest-coverage-mcp

# Test coverage parser
cd coverage-parser
cargo test

# Generate test coverage (in Python project)
coverage run -m pytest tests
coverage json --show-contexts --pretty

# Continue implementation...
```

## Generated Coverage Format

Expects pytest coverage.json with contexts enabled:
```json
{
  "files": {
    "my_project/main.py": {
      "contexts": {
        "15": ["tests.test_main.test_analytics_helper"]
      }
    }
  }
}
```

## Team Adoption

**For Engineers:**
- Delegate test writing to LLMs with confidence
- Verify test coverage attribution precisely  
- Eliminate superficial coverage optimization

**For LLMs:**
- Query specific behavior coverage before writing tests
- Validate test scope after implementation
- Access fresh attribution data during iterative sessions

## Architecture Philosophy

**Atomic and Composable**: Stories and resources designed as building blocks  
**Semantic Interfaces**: URIs communicate intent unambiguously  
**Behavior-First**: Attribution data prioritizes behavior validation over metrics  
**Machine-Readable**: All interfaces optimized for LLM consumption  

---

*Built through human engineering with LLM mentorship - demonstrating collaborative learning approaches for machine learning teams adopting AI-assisted development workflows.*
