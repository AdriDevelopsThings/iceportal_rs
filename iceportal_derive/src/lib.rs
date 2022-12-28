use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(response_object))]
struct Opts {
    url: String
}

#[proc_macro_derive(ResponseObject, attributes(response_object))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Set url for ResponseObject");
    let url = opts.url;
    if url.is_empty() {
        panic!("Set url for ResponseObject");
    }
    let DeriveInput { ident, .. } = input;
    let output = quote! {
        impl ResponseObject for #ident {
            fn fetch(fetcher: Fetcher, options: Option<HashMap<&str, &str>>) -> Result<#ident, reqwest::Error> {
                fetcher.fetch(options)
            }
        
            fn url() -> &'static str {
                #url
            }
        }
    };
    output.into()
}