use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "parser/foolhtml.pest"]
struct SHParser;

use super::ast_types::{Node, Elem, Cont, Attr};

pub fn from_str(input: &str) -> Vec<Node>{
    generate(input)
}

fn generate(input: &str) -> Vec<Node> {
    let mut ast : Vec<Node> = Vec::new();
    let parse_res = SHParser::parse(Rule::html, input).expect("unsuccessful parse");
    for item in parse_res {
        let new_node = gen_node(item);
        ast.push(new_node)
    }
    ast
}

fn gen_node(val: Pair<Rule>) -> Node {
    Node::ELEM(gen_elem(val))
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
            //Couldn't figure out in the .pest file how to use the same rule for both
            Rule::n_attr_val | Rule::q_attr_val => attr.value.push_str(v.as_str()),
            _ => unreachable!()
        }
    }
    match elem.attr{
        Some(ref mut vec) => {vec.push(attr);},
        None => elem.attr=  Some(vec![attr]),
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
        Some(ref mut vec) => {vec.push(gen_node(val));},
        None => elem.children = Some(vec![gen_node(val)]),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::{string_vec, node_el_vec};

    ///Helper macro to test a string against the expected output of elements
    ///Specify the expected elements inside []
    macro_rules! test_str_elems {
        ($input:literal, [ $($expected:expr),+ ]) => {
            let output = from_str($input);
            assert_eq!(output, vec![$(Node::ELEM($expected)),+]);
        }
    }

    #[test]
    fn parses_simple_tag() {
        test_str_elems!("hello", [Elem::from_ta("hello")]);
    }

    #[test]
    fn parses_two_simple_tags() {
        test_str_elems!("hello\nworld", [Elem::from_ta("hello"), Elem::from_ta("world")]);
    }

    #[test]
    fn parses_simple_content() {
        test_str_elems!("hello world", [Elem::from_ta_col("hello", "world")]);
    }

    #[test]
    fn parses_single_child() {
        test_str_elems!("hello\n  world",
                        [Elem::from_ta_ch("hello", node_el_vec![Elem::from_ta("world")])]);
    }

    #[test]
    fn parses_two_children() {
        test_str_elems!("hello\n  world\n  today",
                        [Elem::from_ta_ch("hello",
                                          node_el_vec![Elem::from_ta("world"),
                                                       Elem::from_ta("today")])]);
    }

    #[test]
    fn parses_two_level_children() {
        test_str_elems!("hello\n  world\n  today\n    tomorrow\n  always",
                        [Elem::from_ta_ch("hello",
                                          node_el_vec![Elem::from_ta("world"),
                                                       Elem::from_ta_ch("today",
                                                                        node_el_vec![Elem::from_ta("tomorrow")]),
                                                       Elem::from_ta("always")])]);
    }

    #[test]
    fn parses_single_class() {
        test_str_elems!("hello.world-fam", [Elem::from_ta_cl("hello", string_vec!["world-fam"])]);
    }

    #[test]
    fn parses_single_char_class() {
        test_str_elems!("hello.w", [Elem::from_ta_cl("hello", string_vec!["w"])]);
    }

    #[test]
    fn parses_two_classes() {
        test_str_elems!("hello.world.fam", [Elem::from_ta_cl("hello", string_vec!["world", "fam"])]);
    }

    #[test]
    fn parses_single_id() {
        test_str_elems!("hello#world-class-nr1", [Elem::from_ta_id("hello", "world-class-nr1")]);
    }

    #[test]
    fn parses_single_char_id() {
        test_str_elems!("hello#w", [Elem::from_ta_id("hello", "w")]);
    }

    #[test]
    fn parses_id_and_classes() {
        test_str_elems!("hello#world.not.today", [Elem::from_ta_id_cl("hello", "world", string_vec!["not", "today"])]);
    }

    #[test]
    fn parses_multi_line_content() {
        test_str_elems!("hello:\n  bon\n  jour", [Elem::from_ta_cob("hello", string_vec!["bon", "jour"])]);
    }

    #[test]
    fn parses_id_and_class_on_block_el() {
        test_str_elems!("hello#world.today:\n  friends",
                        [Elem::from_ta_id_cl_cob("hello", "world",
                                                 string_vec!["today"],
                                                 string_vec!["friends"])]);
    }

    #[test]
    fn parses_simple_attribute() {
        test_str_elems!(r#"hello world="great""#,
                        [Elem::from_ta_at("hello",
                                          vec![Attr{ name: "world".to_string(),
                                                     value: "great".to_string()}])]);
    }

    #[test]
    fn parses_attribute_with_single_quotes() {
        test_str_elems!(r#"hello world='Mr. "Anderson"'"#,
                        [Elem::from_ta_at("hello",
                                          vec![Attr{ name: "world".to_string(),
                                                     value: "Mr. \"Anderson\"".to_string()}])]);
    }

    #[test]
    fn parses_naked_attribute_no_quotes() {
        test_str_elems!("hello world=great",
                        [Elem::from_ta_at("hello",
                                          vec![Attr{ name: "world".to_string(),
                                                     value: "great".to_string()}])]);
    }

    #[test]
    fn parses_naked_attribute_with_children() {
        test_str_elems!("hello world=great\n  today",
                        [Elem::from_ta_at_ch("hello",
                                             vec![Attr{ name: "world".to_string(),
                                                        value: "great".to_string()}],
                                             node_el_vec![Elem::from_ta("today")])]);
    }

    #[test]
    fn parses_naked_attribute_block() {
        test_str_elems!("hello world=great:\n  good\n  morning",
                        [Elem::from_ta_at_cob("hello",
                                              vec![Attr{ name: "world".to_string(),
                                                         value: "great".to_string()}],
                                              string_vec!["good", "morning"])]);
    }

    #[test]
    fn parses_attribute_block() {
        test_str_elems!("hello world=\"great day\":\n  good\n  morning",
                        [Elem::from_ta_at_cob("hello",
                                              vec![Attr{ name: "world".to_string(),
                                                         value: "great day".to_string()}],
                                              string_vec!["good", "morning"])]);
    }
}
