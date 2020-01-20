extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_str, Data, VisPublic, Visibility, DataStruct, Fields};

#[proc_macro_derive(Dumb)]
pub fn dumb_macro_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = parse_str(&s).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let dumb_name = format_ident!("Dumb{}", &ast.ident);
    let name = &ast.ident;
    let mut dumb_ast = ast.to_owned();
    dumb_ast.ident = dumb_name.clone();
    let new_dumb_ast = dumb_ast.clone();
    let fields = match &new_dumb_ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    match dumb_ast.data {
        Data::Struct(ref mut data_struct) => {
            for field in &mut data_struct.fields {
                field.vis = Visibility::Public(VisPublic { pub_token: syn::token::Pub::default() });
            }
        }
        _ => panic!("only struct supported"),
    };
    let res = quote! {
        #dumb_ast
        impl #name {
            pub fn dumb(self) -> #dumb_name {
                #dumb_name {
                    #(#field_name: self.#field_name),*
                }
            }
        }
    };
    TokenStream::from(res)
}
