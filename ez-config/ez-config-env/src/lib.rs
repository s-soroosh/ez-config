use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    Data, DeriveInput, Fields, GenericParam, Generics, Index, parse_macro_input, parse_quote,
};
use syn::spanned::Spanned;

#[proc_macro_derive(EnvConfig)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("parse_macro_input:{:#?}!", input);
    let nested_output = generate_from_env(&input.data);

    let ident = input.ident;
    // let msg = quote! {nested_output};
    println!("msg is {:#?}", nested_output);
    let output = quote! {
        impl #ident {
            // #[doc = #msg]
            fn from_env() -> Self {
                #ident {
                    #(#nested_output),*
                }
            }
        }
    };
    output.into()
}


fn generate_from_env(data: &Data) -> Vec<TokenStream> {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    println!("fields are {:#?}", fields.named);

                    let result: Vec<TokenStream> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let env_variable = format!("{}", name.as_ref().unwrap().to_string());
                        let is_string = if let syn::Type::Path(ref p) = f.ty {
                            p.path.segments.len() == 1 && p.path.segments[0].ident == "String"
                        } else { false };
                        quote! {
                            #name: ::std::env::var(#env_variable).unwrap()
                        }
                    }).collect();
                    result
                }
                _ => {
                    panic!("Just named attributes please")
                }
            }
        }
        _ => unimplemented!(),
    }
}