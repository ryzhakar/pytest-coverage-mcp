pub mod attribution_structures;

use coverage_parser::types::*;
use std::collections::{HashMap, HashSet};
use crate::attribution_structures::*;

pub type AttributionMapping = HashMap<
    attribution_structures::SourceElement,
    HashMap<
        CoverageMarks,
        LineNumberVector,
    >,
>;

pub struct AttributionEngine {
    pub full_accumulated_attribution: AttributionMapping
}
impl AttributionEngine {
    pub fn new(
        module_map: RawAttributionMap,
        class_map: RawAttributionMap,
        func_map: RawAttributionMap,
        py_test_dir: &str,
    ) -> Self {
        let mut module_attribution = Self::construct_attribution_mapping_from(
            module_map,
            Some(SourceElementType::Module),
            py_test_dir,
        );
        let mut class_attribution = Self::construct_attribution_mapping_from(
            class_map,
            Some(SourceElementType::Class),
            py_test_dir,
        );
        let mut func_attribution = Self::construct_attribution_mapping_from(
            func_map,
            Some(SourceElementType::FunctionLike),
            py_test_dir,
        );

        // Aggregate func attributions into classes
        Self::accumulate_subattribution(
            &mut class_attribution,
            &func_attribution,
        );
        // Aggregate (already aggregated!) class attributions into modules
        Self::accumulate_subattribution(
            &mut module_attribution,
            &class_attribution,
        );

        Self::dedup(&mut module_attribution);
        Self::dedup(&mut class_attribution);
        Self::dedup(&mut func_attribution);

        Self {
            full_accumulated_attribution: module_attribution.into_iter()
            .chain(class_attribution)
            .chain(func_attribution)
            .collect::<AttributionMapping>()
        }
    }

    // remove duplicate lines and sort line vectors
    fn dedup(attribution: &mut AttributionMapping) {
        for coverage in attribution.values_mut() {
            for (test, lines) in coverage.clone() {
                let mut lines = lines
                    .into_iter()
                    .collect::<HashSet<u32>>()
                    .into_iter()
                    .collect::<Vec<u32>>();
                lines.sort_unstable();
                coverage.insert(test, lines);
            }
        }
    }

    fn construct_attribution_mapping_from(
        raw_map: RawAttributionMap,
        element_type_override: Option<SourceElementType>,
        py_test_dir: &str,
    ) -> AttributionMapping {
        let mut final_map: AttributionMapping = HashMap::new();
        for ((raw_file_path, raw_source_path), contexts) in raw_map {
            let mut coverage_map: HashMap<CoverageMarks, LineNumberVector> = HashMap::new();
            let source = SourceElement::from_parts(
                raw_file_path.clone(),
                raw_source_path,
                element_type_override.clone(),
            );
            for (serline, test_paths) in contexts {
                let line: u32;
                match serline.parse::<u32>() {
                    Ok(number) => { line = number },
                    // It really should parse, but maybe there could be empty strings or smth
                    _ => {continue}
                }
                for raw_test_path in test_paths {
                    let test = if raw_test_path.is_empty() {
                        CoverageMarks::Uncovered
                    } else {
                        let internal_test_element = TestElement::from_parts(raw_test_path, py_test_dir);
                        if internal_test_element.is_in_test_dir {
                            CoverageMarks::ExplicitlyCoveredBy(internal_test_element)
                        } else {
                            CoverageMarks::ImplicitlyCoveredBy(internal_test_element)
                        }
                    };
                    match coverage_map.get_mut(&test) {
                        Some(lines) => {lines.push(line);},
                        None => {
                            let lines = vec![line];
                            coverage_map.insert(test, lines);
                        },
                    };
                }
            }
            final_map.insert(source, coverage_map);
        }
        final_map
    }

    fn accumulate_subattribution(
        accumulating_map: &mut AttributionMapping,
        donoring_map: &AttributionMapping,
    ) {
        for (acc_source, acc_coverage) in accumulating_map {
            for (don_source, don_coverage) in donoring_map {
                if !acc_source.does_match_at_start_of(don_source) {
                    continue;
                }
                for (don_test, don_lines) in don_coverage {
                    match acc_coverage.get_mut(don_test) {
                        Some(acc_coverage_lines) => {
                            acc_coverage_lines.extend(don_lines.clone());
                        },
                        None => {
                            acc_coverage.insert(don_test.clone(), don_lines.clone());
                        }
                    };
                    
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
