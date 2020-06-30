use proc_macro2::TokenStream;
use quote::{quote};
use foolhtml_shared::{renderer, parser::ast};
use syn;

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let fields = get_named_fields(&input.data);
    let args = gen_format_args(fields);

    let config = get_template_config(input.attrs);
    let source = config.source;
    let ast = ast::from_str(&source);
    let mut html = renderer::render(ast);
    //TODO find better way to handle variables and allow single {} in content
    html = html.replace("{{", "{");
    html = html.replace("}}", "}");

    let tokens = quote!{
        impl#generics Template for #name#generics {
            fn render(&self) -> String {
                format!(#html, #args)
            }
        }
    };
    tokens.into()
}

#[derive(Debug, Default)]
struct TemplateConfig {
    source: String,
}

fn get_template_config(attrs: Vec<syn::Attribute>) -> TemplateConfig {
    let mut ident_val_pairs = Vec::<(String, String)>::new();
    for attr in attrs {
        let mut ident = String::default();
        let mut val = String::default();
        for token_tree in attr.tokens {
             match token_tree {
                 proc_macro2::TokenTree::Group(gp) => {
                     for token in  gp.stream() {
                         match token {
                             proc_macro2::TokenTree::Ident(i) => ident =  i.to_string(),
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
    let mut config = TemplateConfig::default();
    for (ident, val) in ident_val_pairs {
        if ident == "source" { config.source = val; }
    }
    config
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
