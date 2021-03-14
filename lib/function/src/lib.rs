use proc_macro::TokenStream;

#[proc_macro]
pub fn export(item: TokenStream) -> TokenStream {
    item
}
