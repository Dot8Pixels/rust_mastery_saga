use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_visibility = &input.vis;
    let fn_signature = &input.sig;
    let fn_body = &input.block;

    let expanded = quote! {
        #fn_visibility #fn_signature {
            let start = std::time::Instant::now();
            let result = (|| #fn_body)();
            let elapsed = start.elapsed();
            println!("Function '{}' took {:?}", stringify!(#fn_name), elapsed);
            result
        }
    };

    TokenStream::from(expanded)
}
