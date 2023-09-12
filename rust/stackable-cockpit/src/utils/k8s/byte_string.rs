use std::{error::Error, fmt::Display, string::FromUtf8Error};

use k8s_openapi::ByteString;

/// The [`ByteStringExt`] enables [`ByteString`] to be converted to a [`String`].
pub trait ByteStringExt {
    type Error: Error + Display;

    fn try_to_string(&self) -> Result<String, Self::Error>;
}

impl ByteStringExt for ByteString {
    type Error = FromUtf8Error;

    fn try_to_string(&self) -> Result<String, Self::Error> {
        // NOTE (Techassi): This extension can possible be moved to `k8s-openapi`.
        // First we need to make sure the [`ByteString`] data is UTF-8 data, and
        // UTF-8 data only.
        String::from_utf8(self.0.to_owned())
    }
}
