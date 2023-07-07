use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;
use snafu::ResultExt;

use super::{Result, TemplatingSnafu, YamlSnafu};

pub trait Parser {
    type Input;
    type Output;
    fn parse(&self, input: Self::Input) -> Result<Self::Output>;
    fn then<P2: Parser<Input = Self::Output>>(self, other: P2) -> Chain<Self, P2>
    where
        Self: Sized,
    {
        Chain(self, other)
    }
}

pub struct Text;
impl Parser for Text {
    type Input = String;
    type Output = String;

    fn parse(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(input)
    }
}
pub struct Tera<'a> {
    pub parameters: &'a HashMap<String, String>,
}
impl<'a> Parser for Tera<'a> {
    type Input = String;
    type Output = String;
    fn parse(&self, input: Self::Input) -> Result<Self::Output> {
        let mut context = tera::Context::new();
        for (name, value) in self.parameters {
            context.insert(name, value)
        }
        tera::Tera::one_off(&input, &context, true).context(TemplatingSnafu)
    }
}
pub struct Yaml<T>(PhantomData<T>);
impl<T: DeserializeOwned> Parser for Yaml<T> {
    type Input = String;
    type Output = T;

    fn parse(&self, input: Self::Input) -> Result<Self::Output> {
        serde_yaml::from_str(&input).context(YamlSnafu {})
    }
}
impl<T> Default for Yaml<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub struct Chain<P1, P2>(P1, P2);
impl<P1, P2> Parser for Chain<P1, P2>
where
    P1: Parser,
    P2: Parser<Input = P1::Output>,
{
    type Input = P1::Input;
    type Output = P2::Output;

    fn parse(&self, input: Self::Input) -> Result<Self::Output> {
        self.1.parse(self.0.parse(input)?)
    }
}
