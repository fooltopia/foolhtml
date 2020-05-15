#![allow(dead_code)] // TODO remove
mod parser;
mod renderer;
mod util;

use parser::ast;

pub fn render_static_template_str(input: &str) -> String {
    let tree = ast::from_str(input);
    renderer::render(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_single_tag_template() {
        let output = render_static_template_str("hello");
        assert_eq!(output, "<hello />")
    }

    #[test]
    fn renders_two_tags_template() {
        let output = render_static_template_str("br\nbr");
        assert_eq!(output, "<br /><br />")
    }

    #[test]
    fn renders_tag_content_template() {
        let output = render_static_template_str("p hello world");
        assert_eq!(output, "<p>hello world</p>")
    }

    #[test]
    fn renders_tag_block_content_template() {
        let output = render_static_template_str("p:\n  how\n  are\n  you?");
        assert_eq!(output, "<p>how<br>are<br>you?</p>")
    }

    #[test]
    fn renders_tag_tag_id_attributes_content() {
        let output = render_static_template_str("div#title.big lang=en How are you?");
        assert_eq!(output, "<div id=\"title\" class=\"big\" lang=\"en\">How are you?</div>")
    }

    #[test]
    fn renders_tag_tag_id_attributes_children() {
        let input = "div#greeting.fancy type=\"Friend's Hello\"
  p#question.informal lang=en:
    How are you, mate?";
        println!("{}", input);
        let output = render_static_template_str(input);
        let expected = "<div id=\"greeting\" class=\"fancy\" type=\"Friend's Hello\">\
                          <p id=\"question\" class=\"informal\" lang=\"en\">How are you, mate?</p>\
                          </div>";
        assert_eq!(output, expected)
    }

    #[test]
    fn renders_complex_static_template_str() {
       let input =  "h1#title.fancy.large Hello World
div
  img#title-image src=\"images/title.jpg\" width=1000 height=300 alt=\"A great title image.\"";
        let output = render_static_template_str(input);
        let expected = "<h1 id=\"title\" class=\"fancy large\">Hello World</h1>\
                        <div>\
                        <img id=\"title-image\" src=\"images/title.jpg\" width=\"1000\" height=\"300\" alt=\"A great title image.\" />\
                        </div>";
        assert_eq!(output, expected)
    }
}
