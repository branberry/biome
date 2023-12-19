use crate::prelude::*;
use biome_css_syntax::CssMediaQueryConsequent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryConsequent;
impl FormatNodeRule<CssMediaQueryConsequent> for FormatCssMediaQueryConsequent {
    fn fmt_fields(&self, node: &CssMediaQueryConsequent, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}