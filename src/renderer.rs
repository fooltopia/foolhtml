use crate::parser::data_structs::Elem;

pub fn render(input: Vec<Elem>) -> String {
    let mut result = String::new();
    for elem in input{
        match elem.cont {
            Some(c) => { let el_str = format!("<{0}>{1}</{0}>", elem.tag, c);
                         result.push_str(&el_str); }
            None => { let el_str = format!("<{}/>", elem.tag);
                      if result.len() > 0 {result.push_str("\n")}
                      result.push_str(&el_str); }
        }
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
        assert_eq!(output, "<hello/>\n<world/>");
    }

    #[test]
    fn renders_tag_cont() {
        let output = render(vec![Elem::from_ta_col("hello", "world")]);
        assert_eq!(output, "<hello>world</hello>")
    }
}
