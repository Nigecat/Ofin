use quote::quote;
use proc_macro::TokenStream;
use syn::Item;

#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
    let src: syn::File = syn::parse(input).unwrap();

    // Store the (name, argument count) of each function
    let mut functions: Vec<(String, usize)> = Vec::new();  
    // Store the name of any submodules 
    let mut submodules: Vec<String> = Vec::new();

    for item in &src.items {
        if let Item::Fn(f) = &item {
            let name = f.sig.ident.to_string();
            let args = f.sig.inputs.len();
        
            functions.push((name, args));
        }

        // else if let Item::Mod(m) = &item {
        //     let name = m.ident.to_string();

        //     submodules.push(name);
        // }
    }

    let names = functions.iter().map(|(name, _)| name);
    let args = functions.iter().map(|(_, arg)| arg);
    
    let expanded = quote! {
        pub static __FUNCTION_MAP: &[(&'static str, usize)] = &[
            #((#names, #args)),*
        ];

        #src
    };

    expanded.into()
}
