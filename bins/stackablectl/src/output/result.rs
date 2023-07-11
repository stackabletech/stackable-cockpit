use comfy_table::{presets::UTF8_FULL, ContentArrangement, Row, Table};
use serde::Serialize;

use crate::cli::OutputType;

pub trait ResultOutput: Serialize + TabledOutput + Sized {
    type Error: std::error::Error + From<serde_json::Error> + From<serde_yaml::Error>;
    const EMPTY_MESSAGE: &'static str = "No entries";

    fn output(&self, output_type: OutputType) -> Result<String, Self::Error> {
        match output_type {
            OutputType::Plain => self.plain_output(),
            OutputType::Json => self.json_output(),
            OutputType::Yaml => self.yaml_output(),
        }
    }

    fn plain_output(&self) -> Result<String, Self::Error> {
        if self.rows().is_empty() {
            return Ok(Self::EMPTY_MESSAGE.into());
        }

        // Build the base table
        let mut table = Table::new();

        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(Self::PRESET);

        // Get columns and rows
        let columns = Self::COLUMNS;
        let rows = self.rows();

        // Only add a header when we have columns
        if !columns.is_empty() {
            table.set_header(columns);
        }

        // Add rows to table
        table.add_rows(rows);

        Ok(table.to_string())
    }

    fn json_output(&self) -> Result<String, Self::Error> {
        Ok(serde_json::to_string(&self)?)
    }

    fn yaml_output(&self) -> Result<String, Self::Error> {
        Ok(serde_yaml::to_string(&self)?)
    }
}

pub trait TabledOutput {
    /// Preset used by the table output.
    const PRESET: &'static str = UTF8_FULL;

    /// Columns needed to render that table.
    const COLUMNS: &'static [&'static str] = &[];

    /// Type of row.
    type Row: Into<Row>;

    /// Returns the rows to be inserted into the table.
    fn rows(&self) -> Vec<Self::Row>;
}

#[cfg(test)]
mod test {
    use crate::cli::OutputType;

    use super::{ResultOutput, TabledOutput};
    use serde::Serialize;
    use snafu::Snafu;

    #[test]
    fn basic_output() {
        #[derive(Debug, Serialize)]
        struct Data {
            foo: String,
            bar: String,
        }

        #[derive(Debug, Snafu)]
        pub enum DataError {
            #[snafu(display("unable to format yaml output"), context(false))]
            YamlOutputFormatError { source: serde_yaml::Error },

            #[snafu(display("unable to format json output"), context(false))]
            JsonOutputFormatError { source: serde_json::Error },
        }

        let d = Data {
            foo: "foo value".into(),
            bar: "bar value".into(),
        };

        impl ResultOutput for Data {
            type Error = DataError;
        }

        impl TabledOutput for Data {
            const COLUMNS: &'static [&'static str] = &["NAME", "VALUE"];

            type Row = Vec<String>;

            fn rows(&self) -> Vec<Self::Row> {
                vec![
                    vec!["foo".into(), self.foo.clone()],
                    vec!["bar".into(), self.bar.clone()],
                ]
            }
        }

        // Table output
        let expected = "
┌──────┬───────────┐
│ NAME ┆ VALUE     │
╞══════╪═══════════╡
│ foo  ┆ foo value │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ bar  ┆ bar value │
└──────┴───────────┘";

        assert_eq!(
            "\n".to_string() + &d.output(OutputType::Plain).unwrap(),
            expected
        );

        // JSON output
        let expected = "{\"foo\":\"foo value\",\"bar\":\"bar value\"}";
        assert_eq!(d.output(OutputType::Json).unwrap(), expected);

        // YAML output
        let expected = "foo: foo value\nbar: bar value\n";
        assert_eq!(d.output(OutputType::Yaml).unwrap(), expected);
    }
}
