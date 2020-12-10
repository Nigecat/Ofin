extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(VariantCount)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (visibility, ident, generics) = (&ast.vis, &ast.ident, &ast.generics);

    // Make sure this is being derived on an enum
    let variants = match &ast.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("VariantCount can only be derived on enums"),
    };

    // Count the number of variants
    let count = variants.len();

    // Declare the variant count as a constant on the enum as enum::VARIANT_COUNT
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let tokens = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #visibility const VARIANT_COUNT: usize = #count;
        }
    };
    tokens.into()
}
