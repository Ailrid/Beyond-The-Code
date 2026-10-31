use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Write},
};

use comrak::{
    Node,
    html::{ChildRendering, Context},
    nodes::*,
};

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }

    #[inline]
    pub fn formater<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        match node.data().value {
            // Commonmark
            NodeValue::BlockQuote => self.render_block_quote(context, node, entering),
            NodeValue::Code(ref nc) => self.render_code(context, node, entering, nc),
            NodeValue::CodeBlock(ref ncb) => self.render_code_block(context, node, entering, ncb),
            NodeValue::Document => Ok(ChildRendering::HTML),
            NodeValue::Emph => self.render_emph(context, node, entering),
            NodeValue::Heading(ref nh) => self.render_heading(context, node, entering, nh),
            NodeValue::HtmlBlock(ref nhb) => self.render_html_block(context, entering, nhb),
            #[cfg(feature = "phoenix_heex")]
            NodeValue::HeexBlock(ref nhb) => self.render_heex_block(context, entering, nhb),
            NodeValue::HtmlInline(ref literal) => {
                self.render_html_inline(context, entering, literal)
            }
            #[cfg(feature = "phoenix_heex")]
            NodeValue::HeexInline(ref literal) => {
                self.render_heex_inline(context, entering, literal)
            }
            NodeValue::Image(ref nl) => self.render_image(context, node, entering, nl),
            NodeValue::Item(_) => self.render_item(context, node, entering),
            NodeValue::LineBreak => self.render_line_break(context, node, entering),
            NodeValue::Link(ref nl) => self.render_link(context, node, entering, nl),
            NodeValue::List(ref nl) => self.render_list(context, node, entering, nl),
            NodeValue::Paragraph => self.render_paragraph(context, node, entering),
            NodeValue::SoftBreak => self.render_soft_break(context, node, entering),
            NodeValue::Strong => self.render_strong(context, node, entering),
            NodeValue::Text(ref literal) => self.render_text(context, entering, literal),
            NodeValue::ThematicBreak => self.render_thematic_break(context, node, entering),

            // GFM
            NodeValue::FootnoteDefinition(ref nfd) => {
                self.render_footnote_definition(context, node, entering, nfd)
            }
            NodeValue::FootnoteReference(ref nfr) => {
                self.render_footnote_reference(context, node, entering, nfr)
            }
            NodeValue::Strikethrough => self.render_strikethrough(context, node, entering),
            NodeValue::Highlight => self.render_highlight(context, node, entering),
            NodeValue::Insert => self.render_insert(context, node, entering),
            NodeValue::Table(_) => self.render_table(context, node, entering),
            NodeValue::TableCell => self.render_table_cell(context, node, entering),
            NodeValue::TableRow(thead) => self.render_table_row(context, node, entering, thead),
            NodeValue::TaskItem(ref nti) => self.render_task_item(context, node, entering, nti),

            // Extensions
            NodeValue::Alert(ref alert) => self.render_alert(context, node, entering, alert),
            NodeValue::Callout(ref nc) => self.render_callout(context, node, entering, nc),
            NodeValue::CalloutTitle => self.render_callout_title(context, node, entering),
            NodeValue::DescriptionDetails => {
                self.render_description_details(context, node, entering)
            }
            NodeValue::DescriptionItem(_) => Ok(ChildRendering::HTML),
            NodeValue::DescriptionList => self.render_description_list(context, node, entering),
            NodeValue::DescriptionTerm => self.render_description_term(context, node, entering),
            NodeValue::Escaped => self.render_escaped(context, node, entering),
            NodeValue::EscapedTag(net) => self.render_escaped_tag(context, net),
            NodeValue::FrontMatter(_) => Ok(ChildRendering::HTML),
            NodeValue::Math(ref nm) => self.render_math(context, node, entering, nm),
            NodeValue::MultilineBlockQuote(_) => {
                self.render_multiline_block_quote(context, node, entering)
            }
            NodeValue::Raw(ref literal) => self.render_raw(context, entering, literal),
            #[cfg(feature = "shortcodes")]
            NodeValue::ShortCode(ref nsc) => self.render_short_code(context, entering, nsc),
            NodeValue::SpoileredText => self.render_spoiler_text(context, node, entering),
            NodeValue::Subscript => self.render_subscript(context, node, entering),
            NodeValue::Superscript => self.render_superscript(context, node, entering),
            NodeValue::Underline => self.render_underline(context, node, entering),
            NodeValue::WikiLink(ref nwl) => self.render_wiki_link(context, node, entering, nwl),
            NodeValue::Subtext => self.render_subtext(context, node, entering),
            NodeValue::BlockDirective(ref nbd) => {
                self.render_block_directive(context, node, entering, nbd)
            }
            NodeValue::CommentInline(_) | NodeValue::MultilineComment(_) => {
                Ok(ChildRendering::Skip)
            }
            _ => {
                todo!()
            }
        }
    }

    // Commonmark

    /// Renders sourcepos data for the given node to the supplied [`Context`].
    ///
    /// This function renders anything iff `context.options.render.sourcepos` is
    /// true, and includes a leading space if so, so you can use it  unconditionally
    /// immediately before writing a closing `>` in your opening HTML tag.
    fn render_sourcepos<T>(&self, context: &mut Context<T>, node: Node<'_>) -> fmt::Result {
        Ok(())
    }

    fn render_block_directive<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nbd: &NodeBlockDirective,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_block_quote<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_code<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nc: &NodeCode,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_code_block<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        ncb: &NodeCodeBlock,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_emph<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_heading<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nh: &NodeHeading,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_html_block<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        nhb: &NodeHtmlBlock,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    #[cfg(feature = "phoenix_heex")]
    fn render_heex_block<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        nhb: &crate::nodes::NodeHeexBlock,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    #[cfg(feature = "phoenix_heex")]
    fn render_heex_inline<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        literal: &str,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_html_inline<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        literal: &str,
    ) -> Result<ChildRendering, fmt::Error> {
        // No sourcepos.

        Ok(ChildRendering::HTML)
    }

    fn render_image<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nl: &NodeLink,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_item<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_line_break<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_link<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nl: &NodeLink,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_list<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nl: &NodeList,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_paragraph<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_soft_break<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_strong<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_text<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        literal: &str,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_thematic_break<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    // GFM

    fn render_footnote_definition<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nfd: &NodeFootnoteDefinition,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_footnote_reference<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nfr: &NodeFootnoteReference,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_strikethrough<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_highlight<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_insert<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_table<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_table_cell<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_table_row<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        thead: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_task_item<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nti: &NodeTaskItem,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    // Extensions

    fn render_alert<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        alert: &NodeAlert,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_description_details<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_callout<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nc: &NodeCallout,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_callout_title<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_description_list<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_description_term<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_escaped<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_escaped_tag<T>(
        &self,
        context: &mut Context<T>,
        net: &str,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    /// Renders a math dollar inline, `$...$` and `$$...$$` using `<span>` to be
    /// similar to other renderers.
    pub fn render_math<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nm: &NodeMath,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    /// Renders a math code block, ```` ```math ```` using `<pre><code>`.
    pub fn render_math_code_block<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        literal: &str,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_multiline_block_quote<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_raw<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        literal: &str,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    #[cfg(feature = "shortcodes")]
    fn render_short_code<T>(
        &self,
        context: &mut Context<T>,
        entering: bool,
        nsc: &NodeShortCode,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_spoiler_text<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_subscript<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_subtext<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_superscript<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_underline<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn render_wiki_link<T>(
        &self,
        context: &mut Context<T>,
        node: Node<'_>,
        entering: bool,
        nwl: &NodeWikiLink,
    ) -> Result<ChildRendering, fmt::Error> {
        Ok(ChildRendering::HTML)
    }

    fn put_footnote_backref<T>(
        &self,
        context: &mut Context<T>,
        nfd: &NodeFootnoteDefinition,
    ) -> Result<bool, fmt::Error> {
        Ok(true)
    }

    /// Check if the input would be considered a dangerous url
    pub fn dangerous_url(input: &str) -> bool {
        return true;
    }

    pub fn escape(output: &mut dyn Write, buffer: &str) -> fmt::Result {
        Ok(())
    }

    pub fn escape_href(output: &mut dyn Write, buffer: &str, relaxed_ipv6: bool) -> fmt::Result {
        Ok(())
    }

    /// Writes an opening HTML tag, using an iterator to enumerate the attributes.
    /// Note that attribute values are automatically escaped.
    pub fn write_opening_tag<K: AsRef<str>, V: AsRef<str>>(
        output: &mut dyn Write,
        tag: &str,
        attributes: impl IntoIterator<Item = (K, V)>,
    ) -> fmt::Result {
        Ok(())
    }
}
