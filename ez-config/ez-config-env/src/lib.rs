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
            fn from_env() {
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
                    // Expands to an expression like
                    //
                    //     0 + self.x.heap_size() + self.y.heap_size() + self.z.heap_size()
                    //
                    // but using fully qualified function call syntax.
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `heap_size_of_children`
                    // call. This way if one of the field types does not
                    // implement `HeapSize` then the compiler's error message
                    // underlines which field it is. An example is shown in the
                    // readme of the parent directory.
                    let result: Vec<TokenStream> = fields.named.iter().map(|f| {
                        let name = &f.ident;

                        let isString = if let syn::Type::Path(ref p) = f.ty {
                            p.path.segments.len() == 1 && p.path.segments[0].ident == "String"
                        } else { false };
                        quote! {
                            println!("Is string? {}", #isString)
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