use std::str::FromStr;

/// Enum into which the output format argument is parsed
#[derive(Debug,PartialEq)]
pub enum OutputFormat {
    Json,
    Default
}
impl FromStr for OutputFormat {
    type Err = ();
    fn from_str(input: &str) -> Result<OutputFormat, Self::Err> {
        match input {
            "json"      => Ok(OutputFormat::Json),
            _           => Ok(OutputFormat::Default),
        }
    }
}
impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = match self {
            OutputFormat::Json => "json",
            OutputFormat::Default => "default"
        };
        f.write_str(&fmt_str)
    }
}