use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlLocalDateTime, TomlLocalDateTimeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlLocalDateTime;

impl FormatNodeRule<TomlLocalDateTime> for FormatTomlLocalDateTime {
    fn fmt_fields(&self, node: &TomlLocalDateTime, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlLocalDateTimeFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
