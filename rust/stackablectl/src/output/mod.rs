use nu_ansi_term::Color::Green;
use snafu::{ResultExt, Snafu};
use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};
use tera::Tera;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create output renderer"))]
    CreationError { source: tera::Error },

    #[snafu(display("failed to render console output"))]
    RenderError { source: tera::Error },
}

pub trait ContextExt {
    fn into_context(self) -> tera::Context;
}

#[derive(Debug, Default)]
pub struct Context {
    pub used_operator_namespace: String,
    pub used_product_namespace: String,

    pub command_hints: Vec<String>,
    pub post_hints: Vec<String>,
    pub pre_hints: Vec<String>,

    pub output: String,
}

impl ContextExt for Context {
    fn into_context(self) -> tera::Context {
        let mut ctx = tera::Context::new();

        ctx.insert("default_operator_namespace", DEFAULT_OPERATOR_NAMESPACE);
        ctx.insert("default_product_namespace", DEFAULT_PRODUCT_NAMESPACE);

        ctx.insert("used_operator_namespace", &self.used_operator_namespace);
        ctx.insert("used_product_namespace", &self.used_product_namespace);

        ctx.insert("command_hints", &self.command_hints);
        ctx.insert("post_hints", &self.post_hints);
        ctx.insert("pre_hints", &self.pre_hints);

        ctx.insert("output", &self.output);

        ctx
    }
}

#[derive(Debug)]
pub struct Output {
    context: Context,
    renderer: Tera,
}

impl Output {
    /// Creates a new centralized [`Output`] facility, which allows printing
    /// unified console output. Internally, it uses a templated renderer which
    /// uses backed in templates to render the console output. The [`Context`]
    /// values can be set using the appropriate associated functions, like
    /// [`Output::add_pre_hint()`].
    pub fn new() -> Result<Self> {
        let mut renderer = Tera::default();
        let context = Context::default();

        renderer
            .add_raw_templates(vec![
                ("result", include_str!("templates/result.tpl")),
                ("error", include_str!("templates/error.tpl")),
            ])
            .context(CreationSnafu)?;

        Ok(Self { renderer, context })
    }

    /// Adds a hint which is printed **before** the main output. This can be
    /// used to display hints or short messages in front of the main content.
    /// Examples are: the current `stackablectl` version, execution time or
    /// the current Kubernetes namespace.
    pub fn add_pre_hint<T>(&mut self, pre_hint: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.context.pre_hints.push(pre_hint.into());
        self
    }

    /// Adds a hint which is printed **after** the main output. This can be
    /// used to display hints or short messages after of the main content.
    /// To print command recommendations, use [`Output::add_command_hint`].
    pub fn add_post_hint<T>(&mut self, post_hint: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.context.post_hints.push(post_hint.into());
        self
    }

    /// Format a command hint. This will produce a sentence like 'Use \<COMMAND>
    /// to \<DESCRIPTION>'. The `description` must start with lowercase letters,
    /// must complete previosly mentioned sentence, and must not end with a period.
    pub fn add_command_hint<C, D>(&mut self, command: C, description: D) -> &mut Self
    where
        C: Into<String>,
        D: Into<String>,
    {
        self.context.command_hints.push(format!(
            "Use {} to {}.",
            Green.bold().paint(format!("\"{}\"", command.into())),
            description.into()
        ));
        self
    }

    pub fn set_operator_namespace<T>(&mut self, namespace: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.context.used_operator_namespace = namespace.into();
        self
    }

    pub fn set_product_namespace<T>(&mut self, namespace: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.context.used_product_namespace = namespace.into();
        self
    }

    pub fn set_output<T>(&mut self, output: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.context.output = output.into();
        self
    }

    pub fn render(self) -> String {
        // TODO (Techassi): Remove unwrap
        self.renderer
            .render("result_success", &self.context.into_context())
            .unwrap()
    }
}

/// Format a command hint. This will produce a sentence like 'Use \<COMMAND>
/// to \<DESCRIPTION>'. The `description` must start with lowercase letters,
/// must complete previosly mentioned sentence, and must not end with a period.
pub fn format_command_hint<T>(command: T, description: T) -> String
where
    T: AsRef<str>,
{
    format!("Use \"{}\" to {}.", command.as_ref(), description.as_ref())
}
