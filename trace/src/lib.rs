use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Expr};

/// Trace a function call
#[proc_macro]
pub fn tracefn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let function = match input {
        Expr::Call(f) => f,
        _ => panic!("Can only trace function calls"),
    };

    let args_ident = (0..function.args.len()).map(|i| format_ident!("__trace_fn_arg_{}", i));
    let args_fmt = std::iter::repeat("{:?}")
        .take(args_ident.len())
        .collect::<Vec<&str>>()
        .join(", ");
    let args_call = args_ident.clone();
    let args_trace = args_ident.clone();
    let f = function.func;
    let args = function.args.into_iter();

    let fname = match *f.clone() {
        Expr::Path(f) => f.path.segments[0].ident.to_string(),
        _ => panic!("Can only trace function calls"),
    };

    let fmt = format!("{}({}) -> {}", "{}", args_fmt, "{:?}");

    let expanded = quote! {
        {
            #(
                let #args_ident = #args;
            )*
            let __trace_fn_result = #f( #( #args_call ),* );
            ::tracing::trace!(#fmt, #fname, #( #args_trace ),*, __trace_fn_result);

            __trace_fn_result
        }
    };

    expanded.into()
}
