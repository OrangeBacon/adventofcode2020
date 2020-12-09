extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, ItemFn, LitStr};


/// annotates a function with a solution struct containing the function
/// and the input data for that day
#[proc_macro_attribute]
pub fn aoc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    let name = ast.sig.ident.clone();

    // the function name is supposed to match /^day(\d{2})$/
    // where the number is between 1 and 25 inclusive

    // emitting compile_error!(...) with name's span causes an error to
    // be emitted at name during compilation

    let name_str = format!("{}", name);
    if name_str.len() != 5 || &name_str[..3] != "day" {
        return TokenStream::from(quote_spanned! {
            name.span() => compile_error!("Invalid aoc function name");
        });
    }

    let day_number = match name_str[3..].parse::<u32>() {
        Ok(a) if a > 0 && a <= 25 => a,
        Err(e) => {
            let msg = format!("Unable to parse aoc day number: {}", e);
            let msg = LitStr::new(&msg, name.span());
            return TokenStream::from(quote_spanned! {
                name.span() => compile_error!(#msg);
            })
        }
        _ => {
            return TokenStream::from(quote_spanned! {
                name.span() => compile_error!("Aoc day number out of range");
            })
        }
    };

    let file = format!("../../data/day{}.txt", day_number);

    let expanded = quote! {
        pub const solution: libaoc::Solution = libaoc::Solution {
            run: #name,
            file: include_str!(#file),
        };

        #ast
    };

    TokenStream::from(expanded)
}
