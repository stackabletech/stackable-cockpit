use serde_yaml::{Mapping, Value};

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
