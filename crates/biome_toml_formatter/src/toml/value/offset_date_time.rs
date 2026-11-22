use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlOffsetDateTime, TomlOffsetDateTimeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlOffsetDateTime;

impl FormatNodeRule<TomlOffsetDateTime> for FormatTomlOffsetDateTime {
    fn fmt_fields(&self, node: &TomlOffsetDateTime, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlOffsetDateTimeFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
