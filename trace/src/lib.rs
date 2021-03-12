use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, Parser};

#[proc_macro_attribute]
pub fn trace(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::Item::parse.parse(input).unwrap();

    let f = match item {
        syn::Item::Fn(f) => f,
        _ => panic!("#[trace] is not supported on non-function items"),
    };

    let syn::ItemFn { attrs, sig, block, vis } = f;
    let body = block.stmts;

    let expanded = quote! {
        #(#attrs)*
        #[::tracing::instrument]
        #vis #sig {
            ::tracing::trace!("->");
            let __trace_result = { #(#body)* };
            ::tracing::trace!("<- {:?}", __trace_result);
            __trace_result
        }
    };
    expanded.into()
}
