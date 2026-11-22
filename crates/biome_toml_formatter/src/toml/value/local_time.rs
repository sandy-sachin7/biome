use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlLocalTime, TomlLocalTimeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlLocalTime;

impl FormatNodeRule<TomlLocalTime> for FormatTomlLocalTime {
    fn fmt_fields(&self, node: &TomlLocalTime, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlLocalTimeFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
