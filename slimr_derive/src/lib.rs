use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use slimr_shared::renderer;
use std::iter::Peekable;
use syn;

#[proc_macro_derive(SlimR, attributes(template))]
pub fn derive_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let opts = Opts::from_derive_input(&input).expect("Wrong Options");
    let name = input.ident;
    let generics = input.generics;
    let fields = get_named_fields(&input.data);
    let args = gen_format_args(fields);

    let mut html = match opts {
        Opts {
            path: Some(..),
            source: Some(..),
        } => panic!("Please only provide one template source"),
        Opts {
            path: Some(path),
            source: _,
        } => renderer::render_path(&path),
        Opts {
            path: _,
            source: Some(source),
        } => renderer::render_source(&source),
        _ => panic!("Please provide either a path or template source code"),
    };

    html = reformat_braces(&html);

    let tokens = quote! {
        impl #generics SlimR for #name #generics {
            fn render(&self) -> String {
                format!(#html, #args)
            }
        }
    };
    tokens.into()
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(template))]
struct Opts {
    path: Option<String>,
    source: Option<String>,
}

fn get_named_fields(data: &syn::Data) -> &syn::FieldsNamed {
    match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn gen_format_args(fields: &syn::FieldsNamed) -> TokenStream {
    let recurse = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name = self.#name
        }
    });
    quote! { #( #recurse ),* }
}

///Turn slimr variables into format! variables
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
    }
    res
}

fn classify_brace<I: Iterator<Item = char>>(
    brace_type: String,
    mut iter: &mut Peekable<I>,
) -> String {
    let mut res = String::new();
    match iter.peek() {
        Some(c) if c.to_string() == brace_type => {
            iter.next();
            res.push_str(&classify_double_brace(brace_type, &mut iter));
        }
        _ => res.push_str(&brace_type.repeat(2)), //one brace becomes two
    }
    res
}

fn classify_double_brace<I: Iterator<Item = char>>(
    brace_type: String,
    mut iter: &mut Peekable<I>,
) -> String {
    let mut res = String::new();
    match iter.peek() {
        Some(c) if c.to_string() == brace_type => {
            res.push_str(&brace_type.repeat(4)); //already saw two braces; this is the third
            res.push_str(&continue_double_braces(brace_type, &mut iter))
        }
        _ => {
            res.push_str(&brace_type);
            return res;
        } // was a double brace, return single
    }
    res
}

fn continue_double_braces<I: Iterator<Item = char>>(
    brace_type: String,
    iter: &mut Peekable<I>,
) -> String {
    let mut res = String::new();
    while let Some(c) = iter.next() {
        if c.to_string() == brace_type {
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
