use crate::parser::data_structs::Elem;

pub fn render(input: Vec<Elem>) -> String {
    let mut result = String::new();
    for elem in input{
        let rendered_el = render_elem(&elem);
        result.push_str(&rendered_el);
    }
    result
}

pub fn render_elem(elem: &Elem) -> String {
    let mut result = String::new();
    let mut opening = String::new();
    let mut content = String::new();

    opening.push_str(&elem.tag);

    if let Some(c) = &elem.cont {
        content.push_str(&c.to_string());
    }
    if let Some(children) = &elem.children {
        for child in children {
            content.push_str(&render_elem(child));
        }
    }

    if content == "" {
        result.push_str(&format!("<{}/>", opening))
    } else {
        result.push_str(&format!("<{op}>{co}</{cl}>", op=opening, co=content, cl=elem.tag))
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_simple_tag() {
        let output = render(vec![Elem::from_ta("hello")]);
        assert_eq!(output, "<hello/>");
    }
    #[test]
    fn renders_two_tags() {
        let output = render(vec![Elem::from_ta("hello"),
                             Elem::from_ta("world")]);
        assert_eq!(output, "<hello/><world/>");
    }

    #[test]
    fn renders_tag_cont() {
        let output = render(vec![Elem::from_ta_col("hello", "world")]);
        assert_eq!(output, "<hello>world</hello>")
    }

    #[test]
    fn renders_tag_children() {
        let output = render(vec![Elem::from_ta_ch("hello", vec![Elem::from_ta("world")])]);
        assert_eq!(output, "<hello><world/></hello>")
    }
}
