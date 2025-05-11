use comrak::{
    adapters::SyntaxHighlighterAdapter, markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins,
};
use html_escape::encode_text;
use std::collections::HashMap;
use std::io::{Result as IoResult, Write};

pub struct TikzAdapter;

impl SyntaxHighlighterAdapter for TikzAdapter {
    fn write_highlighted(
        &self,
        output: &mut dyn Write,
        lang: Option<&str>,
        code: &str,
    ) -> IoResult<()> {
        if let Some("tikz") = lang.map(str::trim) {
            write!(
                output,
                r#"<div class="tikz-figure"><script type="text/tikz">"#
            )?;
            write!(output, "\n{code}</script></div>\n")?;
        } else {
            write!(
                output,
                "<pre><code class=\"language-{}\">{}</code></pre>\n",
                lang.unwrap_or(""),
                encode_text(code)
            )?;
        }
        Ok(())
    }

    fn write_pre_tag(
        &self,
        _output: &mut dyn Write,
        _attributes: HashMap<String, String>,
    ) -> IoResult<()> {
        Ok(())
    }

    fn write_code_tag(
        &self,
        _output: &mut dyn Write,
        _attributes: HashMap<String, String>,
    ) -> IoResult<()> {
        Ok(())
    }
}

pub fn md_to_html(md: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.math_dollars = true;

    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&TikzAdapter);

    markdown_to_html_with_plugins(md, &options, &plugins)
}
