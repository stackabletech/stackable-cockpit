use std::collections::HashMap;

use bcrypt::DEFAULT_COST;
use rand::distributions::{Alphanumeric, DistString};
use serde::de::DeserializeOwned;
use tera::{Context, Function, Tera, Value};

use crate::constants::PASSWORD_LENGTH;

/// Renders the templated `content` by replacing template strings with the
/// appropiate `parameters`. Internally this uses [`tera`] to render the final
/// output. Available helper functions are:
///
/// - `random_password`: Returns a random password with a relatively secure RNG.
///   See [`rand::thread_rng`] for more information.
/// - `bcrypt`: Returns the bcyrpt hash of the provided `password` parameter.
pub fn render(content: &str, parameters: &HashMap<String, String>) -> Result<String, tera::Error> {
    // Create templating context
    let context = Context::from_serialize(parameters)?;

    // Create render engine
    let mut tera = Tera::default();
    tera.register_function("random_password", random_password());
    tera.register_function("bcrypt", bcrypt());

    // Render template
    tera.render_str(content, &context)
}

/// Internal helper function to retrieve value of type `T` from the `map` by
/// `key`. If the `key` is not present in the `map`, it returns an error.
fn get_from_map<T>(map: &HashMap<String, Value>, key: &str) -> tera::Result<T>
where
    T: DeserializeOwned,
{
    match map.get(key) {
        Some(value) => match tera::from_value(value.to_owned()) {
            Ok(extracted) => Ok(extracted),
            Err(_) => Err(format!("Unable to retrieve value of argument {key:?}").into()),
        },
        None => Err(format!("Failed to retrieve missing parameter {key:?}").into()),
    }
}

fn random_password() -> impl Function {
    |_args: &HashMap<String, Value>| -> tera::Result<Value> {
        let password = Alphanumeric.sample_string(&mut rand::thread_rng(), PASSWORD_LENGTH);
        Ok(password.into())
    }
}

fn bcrypt() -> impl Function {
    |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let password: String = get_from_map(args, "password")?;
        let hash = bcrypt::hash(password, DEFAULT_COST)
            .map_err(|err| format!("Failed to create bcrypt hash: {err}"))?;

        Ok(hash.into())
    }
}
