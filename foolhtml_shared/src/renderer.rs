use crate::parser;
use crate::parser::ast_types::{Node, Cont};

pub fn render_source(source: &str) -> String {
    let input = parser::ast::from_str(source);
    render_ast(input)
}

pub fn render_file(path: &str) -> String {
    let source = load_template_file(path);
    render_source(&source)
}

fn render_ast(ast: Vec<Node>) -> String {
    let mut result = String::new();
    for elem in ast{
        let rendered_el = render_elem(&elem);
        result.push_str(&rendered_el);
    }
    result
}

fn load_template_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path)
        .expect(&format!("Couldn't open template file: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}


fn render_elem(node: &Node) -> String {
    let mut result = String::new();
    let mut opening = String::new();
    let mut content = String::new();
    let elem = match node {
        Node::ELEM(el) => el,
        _ => unimplemented!(),
    };

    opening.push_str(&elem.tag);

    if let Some(id) = &elem.id {
        opening.push_str(&format!(r#" id="{}""#, &id));
    }

    if let Some(classes) = &elem.classes {
        opening.push_str(&format!(r#" class="{}""#, classes.join(" ")));
    }

    if let Some(attrs) = &elem.attr {
        for a in attrs {
            opening.push_str(&format!(r#" {}"#, a));
        }
    }

    if let Some(c) = &elem.cont {
        content.push_str(&render_cont(c));
    }
    if let Some(children) = &elem.children {
        for child in children {
            content.push_str(&render_elem(child));
        }
    }

    if content == "" {
        result.push_str(&format!("<{} />", opening))
    } else {
        result.push_str(&format!("<{op}>{co}</{cl}>", op=opening, co=content, cl=elem.tag))
    }
    result
}

fn render_cont(cont: &Cont) -> String {
    match cont {
        Cont::LINE(l) => l.to_string(),
        Cont::BLOCK(b) => {
            b.join("<br>")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast_types::{Elem, Attr};
    use crate::{string_vec, node_el_vec};
    use super::*;

    macro_rules! test_elems {
        ([ $($input:expr),+ ], $expected:literal ) => {
            assert_eq!(render_ast(vec![$(Node::ELEM($input)),+]), $expected);
        }
    }

    #[test]
    fn renders_simple_tag() {
        test_elems!([Elem::from_ta("hello")], "<hello />");
    }

    #[test]
    fn renders_two_tags() {
        test_elems!([Elem::from_ta("hello"),
                     Elem::from_ta("world")], "<hello /><world />");
    }

    #[test]
    fn renders_tag_cont() {
        test_elems!([Elem::from_ta_col("hello", "world")], "<hello>world</hello>");
    }

    #[test]
    fn renders_tag_children() {
        test_elems!([Elem::from_ta_ch("hello", node_el_vec![Elem::from_ta("world")])],
                    "<hello><world /></hello>");
    }

    #[test]
    fn renders_id() {
        test_elems!([Elem::from_ta_id("hello", "world")], "<hello id=\"world\" />");
    }

    #[test]
    fn renders_classes() {
        test_elems!([Elem::from_ta_cl("hello",string_vec!["world", "universe"])],
                    "<hello class=\"world universe\" />");
    }

    #[test]
    fn renders_attributes() {
        test_elems!([Elem::from_ta_at("hello",
                                      vec![Attr{ name: "world".to_string(),
                                                 value: "great".to_string()},
                                           Attr{ name: "sun".to_string(),
                                                 value: "shining".to_string()}])],
                    "<hello world=\"great\" sun=\"shining\" />");
    }

    #[test]
    fn renders_single_quoted() {
        test_elems!([Elem::from_ta_at("img",
                                      vec![Attr{ name: "Mr".to_string(),
                                                 value: "Thomas \"Neo\" Anderson".to_string()}])],
                    "<img Mr='Thomas \"Neo\" Anderson' />");
    }
    #[test]
    fn renders_attributes_on_children() {
        test_elems!([Elem::from_ta_at_ch("hello",
                                         vec![Attr{ name: "world".to_string(),
                                                    value: "great".to_string()}],
                                         node_el_vec![Elem::from_ta_at("how",
                                                                       vec![Attr{ name: "are".to_string(),
                                                                                  value: "you?".to_string()}])])],
                    "<hello world=\"great\"><how are=\"you?\" /></hello>");
    }
}
