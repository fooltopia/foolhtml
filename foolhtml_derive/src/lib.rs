use std::iter::Peekable;
use proc_macro2::TokenStream;
use quote::{quote};
use foolhtml_shared::renderer;
use syn;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let fields = get_named_fields(&input.data);
    let args = gen_format_args(fields);

    let config = get_template_config(input.attrs);
    let mut html = match config.input.unwrap() { //TODO Error Handling
        InputType::PATH(path) => renderer::render_path(&path),
        InputType::SOURCE(string) => renderer::render_source(&string),
    };

    html = reformat_braces(&html);

    let tokens = quote!{
        impl#generics Template for #name#generics {
            fn render(&self) -> String {
                format!(#html, #args)
            }
        }
    };
    tokens.into()
}



#[derive(Debug)]
enum InputType {
    PATH(String),
    SOURCE(String),

}
#[derive(Debug, Default)]
struct TemplateConfig {
    input: Option<InputType>,
}

fn get_template_config(attrs: Vec<syn::Attribute>) -> TemplateConfig {
    let ident_val_pairs = get_identifier_value_pairs(attrs);
    let mut config = TemplateConfig::default();
    for (ident, val) in ident_val_pairs {
        match ident {
            i if i == "source" =>  config.input = Some(InputType::SOURCE(val)),
            i if i == "path" =>  config.input = Some(InputType::PATH(val)),
            _ => unimplemented!(),
        };
    }
    config
}

fn get_identifier_value_pairs(attrs: Vec<syn::Attribute>) -> Vec::<(String, String)> {
    let mut ident_val_pairs = Vec::<(String, String)>::new();
    for attr in attrs {
        let mut ident = String::default();
        let mut val = String::default();
        for token_tree in attr.tokens {
            match token_tree {
                proc_macro2::TokenTree::Group(gp) => {
                    for token in  gp.stream() {
                        match token {
                            proc_macro2::TokenTree::Ident(i) => ident = i.to_string(),
                            proc_macro2::TokenTree::Literal(l) => val = drop_quotes(l.to_string()),
                            _ => (),
                        }
                    }
                },
                _other => unreachable!(), 
            }
        }
        ident_val_pairs.push((ident, val));
    }
    ident_val_pairs
}

///Remove the first and last character of the string
///This is necessary because proc_macro2::TokenTree::Literal
///includes the quotes around the literal string.
fn drop_quotes(s: String) -> String {
    let len = s.len();
    s[1..len-1].to_string()
}

fn get_named_fields(data: &syn::Data) -> &syn::FieldsNamed {
    match *data {
        syn::Data::Struct(ref data) =>
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    fields
                },
                _ => unimplemented!()
            },
        _ => unimplemented!()
    }
}

fn gen_format_args(fields: &syn::FieldsNamed) -> TokenStream {
    let recurse = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name = self.#name
        }
    });
    quote!{ #( #recurse ),* }
}

///Turn fhtml variables into format! variables
///and escape all other curly braces.
fn reformat_braces(source: &str) -> String {
    //TODO find better way to handle variables and allow single {} in content
    let mut res = String::with_capacity(source.len());
    let mut iter = source.chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            '{' => res.push_str(&classify_brace("{".to_string(), &mut iter)),
            '}' => res.push_str(&classify_brace("}".to_string(), &mut iter)),
            _ => res.push(c),
        }
    };
    res
}

fn classify_brace<I:Iterator<Item=char>>(brace_type: String, mut iter :&mut Peekable<I>) -> String {
    let mut res = String::new();
    match iter.peek() {
        Some(c) if c.to_string() == brace_type
            => { iter.next(); res.push_str(&classify_double_brace(brace_type, &mut iter)); },
        _ =>  res.push_str(&brace_type.repeat(2)) , //one brace becomes two
    }
    res
}

fn classify_double_brace<I:Iterator<Item=char>>(brace_type: String, mut iter :&mut Peekable<I>) -> String {
    let mut res = String::new();
    match iter.peek() {
        Some(c) if c.to_string() == brace_type
            => { res.push_str(&brace_type.repeat(4)); //already saw two braces; this is the third
                 res.push_str(&continue_double_braces(brace_type, &mut iter)) },
        _ => {res.push_str(&brace_type); return res }, // was a double brace, return single
    }
    res
}

fn continue_double_braces<I:Iterator<Item=char>>(brace_type: String, iter :&mut Peekable<I>) -> String {
    let mut res = String::new();
    while let Some(c) = iter.next() {
            if c.to_string() == brace_type{
                res.push_str(&brace_type.repeat(2));
            } else {
            break;
            }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::reformat_braces;

    #[test]
    fn formats_single_opening() {
        assert_eq!(reformat_braces("{"), "{{");
    }

    #[test]
    fn formats_double_opening() {
        assert_eq!(reformat_braces("{{"), "{");
    }

    #[test]
    fn formats_multiple_opening() {
        assert_eq!(reformat_braces("{{{"), "{{{{{{");
        assert_eq!(reformat_braces("{{{{"), "{{{{{{{{");
    }

    #[test]
    fn formats_single_closing() {
        assert_eq!(reformat_braces("}"), "}}");
    }

    #[test]
    fn formats_double_closing() {
        assert_eq!(reformat_braces("}}"), "}");
    }


    #[test]
    fn formats_multiple_closing() {
        assert_eq!(reformat_braces("}}}"), "}}}}}}");
        assert_eq!(reformat_braces("}}}}"), "}}}}}}}}");
    }

    #[test]
    fn formats_var() {
        assert_eq!(reformat_braces("{{hello}}"), "{hello}");
    }


    #[test]
    fn formats_open_close_pair() {
        assert_eq!(reformat_braces("{}"), "{{}}");
    }

    // #[test]
    // fn formats_escaped_double_braces() {
    //     //remove the '\' and double braces to escape it format!
    //     assert_eq!(reformat_braces(r#"\{{"#), "{{{{");
    //     //assert_eq!(reformat_braces("\\}}"), "}}}}")
    // }

    // #[test]
    // fn formats_backslash_triple_braces() {
    //     //the backslash should only be removed on double braces
    //     assert_eq!(reformat_braces("\\{{{"), "\\{{{{{{");
    //     assert_eq!(reformat_braces("\\}}}"), "\\}}}}}}")
    // }
}
