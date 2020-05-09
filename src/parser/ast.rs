use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "parser/foolhtml.pest"]
struct SHParser;

use super::data_structs::{Elem,Cont, Attr};

pub fn from_str(input: &str) -> Vec<Elem>{
    generate(input)
}

fn generate(input: &str) -> Vec<Elem> {
    let mut ast : Vec<Elem> = Vec::new();
    let parse_res = SHParser::parse(Rule::html, input).expect("unsuccessful parse");
    for item in parse_res {
        if item.as_rule() == Rule::EOI {
            continue;
        }

        let new_ast_elem = gen_elem(item);
        ast.push(new_ast_elem)
    }
    ast
}

fn gen_elem(val: Pair<Rule>) -> Elem {
    let mut new_elem = Elem::default();
    for val in val.into_inner() {
        match val.as_rule() {
            Rule::tag => new_elem.tag = String::from(val.as_str()),
            Rule::class_name => add_class(& mut new_elem, &val.as_str()),
            Rule::id_name => new_elem.id = Some(String::from(val.as_str())),
            Rule::attr => add_attr(&mut new_elem, val),
            Rule::cont_inline => new_elem.cont = Some(Cont::LINE(String::from(val.as_str()))),
            Rule::cont_block_line => add_cont_block_line(&mut new_elem, &val.as_str()),
            Rule::el => add_child_elems(&mut new_elem, val),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    new_elem
}


fn add_class<'a>(elem: &mut Elem, val: &'a str) {
    match elem.classes {
        Some(ref mut vec) => {vec.push(val.to_string());},
        None => elem.classes =  Some(vec![val.to_string()]),
    }
}

fn add_attr(elem: &mut Elem, val: Pair<Rule>) {
    let mut attr =  Attr::default();
    for v in val.into_inner() {
        match v.as_rule() {
            Rule::attr_name => attr.name.push_str(v.as_str()),
            Rule::attr_val => attr.value.push_str(v.as_str()),
            _ => unreachable!()
        }
    }
    match elem.attributes {
        Some(ref mut vec) => {vec.push(attr);},
        None => elem.attributes =  Some(vec![attr]),
    }
}
fn add_cont_block_line<'a>(elem: &mut Elem, val: &'a str) {
    match elem.cont {
        Some(ref mut cont_enum) => {
            match cont_enum {
                Cont::BLOCK(cont) => cont.push(val.to_string()),
                _ => unreachable!(),}},
        None => elem.cont=  Some(Cont::BLOCK(vec![val.to_string()])),
    }
}

fn add_child_elems<'a>(elem: &mut Elem, val: Pair<'a, Rule>) {
    match elem.children {
        Some(ref mut vec) => {vec.push(gen_elem(val));},
        None => elem.children = Some(vec![gen_elem(val)]),
    }
}



// impl<'a> Eq for Elem {}

#[cfg(test)]
mod tests {
    use super::*;

    /// easily declare a vector of Strings using str literals
    ///...
    /// assert_eq!(vec!["hello".to_string()], string_vec!["hello"])
    ///...
    macro_rules! string_vec {
        ( $( $x:expr ),* ) => {
            vec![$($x.to_string(),)*];
        }
    }


    #[test]
    fn parses_simple_tag() {
        let output = from_str("hello");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], Elem::from_ta("hello"));
    }
    #[test]
    fn parses_two_simple_tags() {
        let output = from_str("hello\nworld");
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], Elem::from_ta("hello"));
        assert_eq!(output[1], Elem::from_ta("world"));
    }
    #[test]
    fn parses_simple_content() {
        let output = from_str("hello world");
        let expected = Elem::from_ta_col("hello", "world");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], expected);
    }

    #[test]
    fn parses_single_child() {
        let output = from_str("hello\n  world");
        let expected = Elem::from_ta_ch("hello", vec![Elem::from_ta("world")]);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], expected);
    }

    #[test]
    fn parses_two_children() {
        let output = from_str("hello\n  world\n  today");
        let expected = Elem::from_ta_ch("hello", vec![Elem::from_ta("world"),
                                                      Elem::from_ta("today")]);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], expected);
    }

    #[test]
    fn parses_two_level_children() {
        let output = from_str("hello\n  world\n  today\n    tomorrow\n  never");
        let expected = Elem::from_ta_ch("hello",
                                        vec![Elem::from_ta("world"),
                                             Elem::from_ta_ch("today",
                                                              vec![Elem::from_ta("tomorrow")]),
                                             Elem::from_ta("never")]);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], expected);
    }

    #[test]
    fn parses_single_class() {
        let output = from_str("hello.world-fam");
        assert_eq!(output, vec![Elem::from_ta_cl("hello", string_vec!["world-fam"])]);
    }

    #[test]
    fn parses_single_char_class() {
        let output = from_str("hello.w");
        assert_eq!(output, vec![Elem::from_ta_cl("hello", string_vec!["w"])]);
    }

    #[test]
    fn parses_two_classes() {
        let output = from_str("hello.world.fam");
        assert_eq!(output, vec![Elem::from_ta_cl("hello", string_vec!["world", "fam"])]);
    }

    #[test]
    fn parses_single_id() {
        let output = from_str("hello#world-class");
        assert_eq!(output, vec![Elem::from_ta_id("hello", "world-class")]);
    }

    #[test]
    fn parses_single_char_id() {
        let output = from_str("hello#w");
        assert_eq!(output, vec![Elem::from_ta_id("hello", "w")]);
    }

    #[test]
    fn parses_id_and_classes() {
        let output = from_str("hello#world.not.today");
        assert_eq!(output, vec![Elem::from_ta_id_cl("hello", "world", string_vec!["not", "today"])]);
    }

    #[test]
    fn parses_multi_line_content() {
        let output = from_str("hello:\n  bon\n  jour");
        assert_eq!(output, vec![Elem::from_ta_cob("hello", string_vec!["bon", "jour"])])
    }

    #[test]
    fn parses_id_and_class_on_block_el() {
        let output = from_str("hello#world.today:\n  friends");
        assert_eq!(output, vec![Elem::from_ta_id_cl_cob("hello", "world",
                                                        string_vec!["today"],
                                                        string_vec!["friends"])]);
    }

    #[test]
    fn parses_simple_attribute() {
        let output = from_str(r#"hello world="great""#);
        assert_eq!(output, vec![Elem::from_ta_at("hello", vec![Attr{ name: "world".to_string(),
                                                                 value: "great".to_string()}])]);
    }
}
