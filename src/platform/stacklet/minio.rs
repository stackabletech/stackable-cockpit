use crate::platform::stacklet::{Product, StackletError};

pub(super) async fn list_products() -> Result<Vec<Product>, StackletError> {
    Ok(vec![])
}
