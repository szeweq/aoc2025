use proc_macro::TokenStream;
use std::env;

#[proc_macro]
pub fn aoc_input(_item: TokenStream) -> TokenStream {
    let pkg_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME must be set");

    // Expected format: "dayXX"
    let day_str = pkg_name
        .strip_prefix("day")
        .expect("Package name must start with 'day'");

    // Construct path: "../../input/XX.txt"
    // We use a relative path from the crate root (where Cargo.toml is).
    // Since the workspace structure is:
    // /
    //   input/
    //     01.txt
    //   day01/
    //     Cargo.toml
    //     src/
    //       main.rs
    //
    // The path relative to `day01/Cargo.toml` (or where `include_str!` is invoked) needs to be correct.
    // `include_str!` paths are relative to the file where it's called.
    // If called in `day01/src/main.rs`, we need `../../input/01.txt`.

    let path = format!("../../input/{}.txt", day_str);

    // Verify file existence at compile time
    // Actually `include_str!` will error if file is missing, satisfying the requirement.

    let expanded = format!("include_str!(\"{}\")", path);

    expanded.parse().unwrap()
}
