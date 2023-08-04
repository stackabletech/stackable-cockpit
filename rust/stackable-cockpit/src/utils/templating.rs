use std::collections::HashMap;

use bcrypt::DEFAULT_COST;
use rand::distributions::{Alphanumeric, DistString};
use tera::{Context, Function, Tera, Value};

use crate::constants::PASSWORD_LENGTH;

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

fn random_password() -> impl Function {
    |_args: &HashMap<String, Value>| -> tera::Result<Value> {
        let password = Alphanumeric.sample_string(&mut rand::thread_rng(), PASSWORD_LENGTH);
        Ok(password.into())
    }
}

fn bcrypt() -> impl Function {
    |args: &HashMap<String, Value>| -> tera::Result<Value> {
        match args.get("password") {
            Some(val) => match tera::from_value::<String>(val.clone()) {
                Ok(password) => {
                    let hash = bcrypt::hash(password, DEFAULT_COST)
                        .map_err(|err| format!("Failed to create bcrypt hash: {err}"))?;
                    Ok(hash.into())
                }
                Err(_) => Err("Cant get value of password".into()),
            },
            None => Err("Parameter password missing".into()),
        }
    }
}
