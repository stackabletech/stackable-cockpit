use serde_yaml::{Mapping, Value};

/// Extracts the merged Helm values for a specific operator from the operator values mapping.
///
/// Looks up the `common` key and the operator-specific key (e.g. `airflow-operator`),
/// then deep-merges them with operator-specific values taking precedence.
pub fn merged_values_for_operator(operator_values: &Mapping, operator_name: &str) -> Mapping {
    let common = operator_values
        .get("common")
        .and_then(Value::as_mapping)
        .cloned()
        .unwrap_or_default();
    let operator_specific = operator_values
        .get(format!("{operator_name}-operator"))
        .and_then(Value::as_mapping)
        .cloned()
        .unwrap_or_default();
    deep_merge(common, operator_specific)
}

/// Deep merges `overlay` into `base`. Overlay values take precedence.
/// When both values are mappings, their contents are merged recursively.
/// Non-mapping values (including sequences) are replaced entirely, not merged.
pub fn deep_merge(mut base: Mapping, overlay: Mapping) -> Mapping {
    for (k, v) in overlay {
        match base.get_mut(&k) {
            Some(base_v) => merge_value(base_v, v),
            None => {
                base.insert(k, v);
            }
        }
    }
    base
}

fn merge_value(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Mapping(base), Value::Mapping(overlay)) => {
            for (k, v) in overlay {
                match base.get_mut(&k) {
                    Some(base_v) => merge_value(base_v, v),
                    None => {
                        base.insert(k, v);
                    }
                }
            }
        }
        (base, overlay) => *base = overlay,
    }
}
