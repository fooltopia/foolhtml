#![allow(dead_code)] // TODO remove
mod parser;
mod renderer;
mod util;

use parser::ast;

pub fn render_static_template(input: &str) -> String {
    let tree = ast::from_str(input);
    renderer::render(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_single_tag_template() {
        let output = render_static_template("hello");
        assert_eq!(output, "<hello/>")
    }

    #[test]
    fn renders_two_tags_template() {
        let output = render_static_template("hello\nworld");
        assert_eq!(output, "<hello/><world/>")
    }

    #[test]
    fn renders_tag_content_template() {
        let output = render_static_template("hello world");
        assert_eq!(output, "<hello>world</hello>")
    }
}
