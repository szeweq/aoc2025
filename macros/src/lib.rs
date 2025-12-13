use proc_macro::TokenStream;
use quote::quote;
use std::env;
use syn::{ItemFn, parse_macro_input};

#[proc_macro]
pub fn aoc_input(_item: TokenStream) -> TokenStream {
    let pkg_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME must be set");
    let day_str = pkg_name
        .strip_prefix("day")
        .expect("Package name must start with 'day'");
    let path = format!("../../input/{}.txt", day_str);
    let expanded = format!("include_str!(\"{}\")", path);
    expanded.parse().unwrap()
}

#[proc_macro_attribute]
pub fn aoc_timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let _fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;

    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_body)();
            let duration = start.elapsed();
            println!("[Duration] {:.2?}", duration);
            result
        }
    };

    TokenStream::from(expanded)
}
