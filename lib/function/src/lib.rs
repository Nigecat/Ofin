use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, Visibility};

#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
    let src: syn::File = syn::parse(input).unwrap();

    // Store the (name, argument count) of each function
    let mut functions: Vec<(String, usize)> = Vec::new();

    for item in &src.items {
        match item {
            Item::Fn(f) => {
                let name = f.sig.ident.to_string();
                let args = f.sig.inputs.len();

                functions.push((name, args));
            }
            Item::Mod(m) => {
                if let Visibility::Public(_) = m.vis {
                    panic!("exporting a submodule is not allowed, please create a wrapper function instead");
                }
            }
            Item::Use(u) => {
                if let Visibility::Public(_) = u.vis {
                    panic!("exporting a submodule is not allowed, please create a wrapper function instead");
                }
            }
            _ => (),
        }
    }

    let names = functions.iter().map(|(name, _)| name);
    let args = functions.iter().map(|(_, arg)| arg);

    let expanded = quote! {
        #[doc(hidden)]
        #[allow(dead_code)]
        #[automatically_derived]
        pub static __FUNCTION_MAP: &[(&'static str, usize)] = &[
            #((#names, #args)),*
        ];

        #src
    };

    expanded.into()
}
