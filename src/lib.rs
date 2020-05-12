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

    #[test]
    fn renders_tag_block_content_template() {
        let output = render_static_template("hello:\n  how\n  are\n  you?");
        assert_eq!(output, "<hello>how<br>are<br>you?</hello>")
    }

    #[test]
    fn renders_tag_tag_id_attributes_content() {
        let output = render_static_template("hello#world.great sun=shining How are you?");
        assert_eq!(output, "<hello id=\"world\" class=\"great\" sun=\"shining\">How are you?</hello>")
    }


    #[test]
    fn renders_tag_tag_id_attributes_children() {
        let input = "hello#world.great sun=shining
  how#are.you today=\"a great day\":
    I'm fine";
        println!("{}", input);
        let output = render_static_template(input);
        let expected = "<hello id=\"world\" class=\"great\" sun=\"shining\">\
                          <how id=\"are\" class=\"you\" today=\"a great day\">I'm fine</how>\
                          </hello>";
        assert_eq!(output, expected)
    }
}
