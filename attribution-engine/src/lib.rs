pub mod attribution_structures;

use coverage_parser::types::*;
use std::collections::{HashMap, HashSet};
use crate::attribution_structures::*;

pub type AttributionMapping = HashMap<
    attribution_structures::SourceElement,
    HashMap<
        attribution_structures::TestElement,
        LineNumberVector,
    >,
>;

pub struct AttributionEngine {}
impl AttributionEngine {
    pub fn new(
        module_map: RawAttributionMap,
        class_map: RawAttributionMap,
        func_map: RawAttributionMap,
    ) -> AttributionMapping {
        let mut module_attribution = Self::construct_attribution_mapping_from(
            module_map,
            Some(SourceElementType::Module),
        );
        let mut class_attribution = Self::construct_attribution_mapping_from(
            class_map,
            Some(SourceElementType::Class),
        );
        let mut func_attribution = Self::construct_attribution_mapping_from(
            func_map,
            Some(SourceElementType::FunctionLike),
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

        return module_attribution.into_iter()
            .chain(class_attribution.into_iter())
            .chain(func_attribution.into_iter())
            .collect::<AttributionMapping>()
    }

    // remove duplicate lines and sort line vectors
    fn dedup(attribution: &mut AttributionMapping) {
        for (_, coverage) in attribution {
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
    ) -> AttributionMapping {
        let mut final_map: AttributionMapping = HashMap::new();
        for ((raw_file_path, raw_source_path), contexts) in raw_map {
            let mut coverage_map: HashMap<TestElement, LineNumberVector> = HashMap::new();
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
                    let test = TestElement::from_parts(raw_test_path);
                    match coverage_map.get_mut(&test) {
                        Some(lines) => {lines.push(line);},
                        None => {
                            let mut lines = vec![];
                            lines.push(line);
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
