use crate::constants::{OPERATOR_HELM_VALUE_BASE_URL, OPERATOR_HELM_VALUE_PATH};

pub fn operator_values_yaml_url(operator_name: &str) -> String {
    format!("{OPERATOR_HELM_VALUE_BASE_URL}{operator_name}-operator/{OPERATOR_HELM_VALUE_PATH}{operator_name}-operator/values.yaml")
}
