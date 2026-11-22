//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlDateTime;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlDateTime;
impl FormatRule<AnyTomlDateTime> for FormatAnyTomlDateTime {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlDateTime, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlDateTime::TomlLocalDate(node) => node.format().fmt(f),
            AnyTomlDateTime::TomlLocalDateTime(node) => node.format().fmt(f),
            AnyTomlDateTime::TomlLocalTime(node) => node.format().fmt(f),
            AnyTomlDateTime::TomlOffsetDateTime(node) => node.format().fmt(f),
        }
    }
}
