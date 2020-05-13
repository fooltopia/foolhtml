use crate::parser::data_structs::{Elem, Cont};

pub fn render(input: Vec<Elem>) -> String {
    let mut result = String::new();
    for elem in input{
        let rendered_el = render_elem(&elem);
        result.push_str(&rendered_el);
    }
    result
}

fn render_elem(elem: &Elem) -> String {
    let mut result = String::new();
    let mut opening = String::new();
    let mut content = String::new();

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
    use crate::parser::data_structs::Attr;
    use crate::string_vec;
    use super::*;

    #[test]
    fn renders_simple_tag() {
        let output = render(vec![Elem::from_ta("hello")]);
        assert_eq!(output, "<hello />");
    }
    #[test]
    fn renders_two_tags() {
        let output = render(vec![Elem::from_ta("hello"),
                             Elem::from_ta("world")]);
        assert_eq!(output, "<hello /><world />");
    }

    #[test]
    fn renders_tag_cont() {
        let output = render(vec![Elem::from_ta_col("hello", "world")]);
        assert_eq!(output, "<hello>world</hello>")
    }

    #[test]
    fn renders_tag_children() {
        let output = render(vec![Elem::from_ta_ch("hello", vec![Elem::from_ta("world")])]);
        assert_eq!(output, "<hello><world /></hello>")
    }

    #[test]
    fn renders_id() {
        let output = render(vec![Elem::from_ta_id("hello", "world")]);
        assert_eq!(output, "<hello id=\"world\" />")
    }

    #[test]
    fn renders_classes() {
        let output = render(vec![Elem::from_ta_cl("hello",string_vec!["world", "universe"])]);
        assert_eq!(output, "<hello class=\"world universe\" />")
    }

    #[test]
    fn renders_attributes() {
        let output = render(vec![Elem::from_ta_at("hello",
                                                  vec![Attr{ name: "world".to_string(),
                                                             value: "great".to_string()},
                                                       Attr{ name: "sun".to_string(),
                                                             value: "shining".to_string()}])]);
        assert_eq!(output, "<hello world=\"great\" sun=\"shining\" />")
    }

    #[test]
    fn renders_single_quoted() {
        let output = render(vec![Elem::from_ta_at("img",
                                                  vec![Attr{ name: "Mr".to_string(),
                                                             value: "Thomas \"Neo\" Anderson".to_string()}])]);
        assert_eq!(output, "<img Mr='Thomas \"Neo\" Anderson' />")
    }
    #[test]
    fn renders_attributes_on_children() {
        let output = render(vec![Elem::from_ta_at_ch("hello",
                                                     vec![Attr{ name: "world".to_string(),
                                                             value: "great".to_string()}],
                                                     vec![Elem::from_ta_at("how",
                                                                           vec![Attr{ name: "are".to_string(),
                                                                                      value: "you?".to_string()}])])]);

        assert_eq!(output, "<hello world=\"great\"><how are=\"you?\" /></hello>")
    }
}
