use markdown::{to_html_with_options, CompileOptions, Constructs, Options, ParseOptions};

pub fn md_to_html(md: &str) -> String {
    let options = Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            gfm_footnote_label: Some("Notes de bas de page".into()),
            gfm_footnote_label_tag_name: Some("h3".into()),
            ..CompileOptions::gfm()
        },
        parse: ParseOptions {
            gfm_strikethrough_single_tilde: true,
            math_text_single_dollar: true,
            constructs: Constructs {
                attention: true,
                autolink: true,
                block_quote: true,
                // character_escape: bool,
                character_reference: true,
                code_indented: true,
                code_text: true,
                definition: true,
                // frontmatter: bool,
                gfm_autolink_literal: true,
                gfm_footnote_definition: true,
                gfm_label_start_footnote: true,
                gfm_strikethrough: true,
                gfm_table: true,
                gfm_task_list_item: true,
                // hard_break_escape: bool,
                // hard_break_trailing: bool,
                heading_atx: true,
                heading_setext: true,
                html_flow: true,
                html_text: true,
                label_start_image: true,
                label_start_link: true,
                label_end: true,
                list_item: true,
                math_flow: true,
                math_text: true,
                mdx_esm: false,
                mdx_expression_flow: false,
                mdx_expression_text: false,
                mdx_jsx_flow: false,
                mdx_jsx_text: false,
                thematic_break: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::gfm()
        },
        ..Options::gfm()
    };
    to_html_with_options(md, &options).unwrap()
}
