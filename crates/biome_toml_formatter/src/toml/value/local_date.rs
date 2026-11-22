use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlLocalDate, TomlLocalDateFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlLocalDate;

impl FormatNodeRule<TomlLocalDate> for FormatTomlLocalDate {
    fn fmt_fields(&self, node: &TomlLocalDate, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlLocalDateFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
