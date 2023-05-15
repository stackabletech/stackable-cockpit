pub mod check;
pub mod params;
pub mod path;
pub mod read;

/// Returns the name of the operator used in the Helm repository.
pub fn operator_name<T>(name: T) -> String
where
    T: AsRef<str>,
{
    format!("{}-operator", name.as_ref())
}
