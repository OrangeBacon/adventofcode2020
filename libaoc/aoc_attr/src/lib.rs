extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Spacing, Punct};
use quote::{quote, quote_spanned, format_ident};
use syn::{
    parse::{ParseStream, Result, Parse},
    parse_macro_input, ItemFn, LitInt, LitStr, Token,
};

struct AocTestAttributes {
    part1: LitStr,
    part2: LitStr,
}

impl Parse for AocTestAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let part1: LitStr = input.parse()?;
        input.parse::<Token!(,)>()?;
        let part2: LitStr = input.parse()?;
        Ok(Self {
            part1: part1,
            part2: part2,
        })
    }
}

/// annotates a function with a solution struct containing the function
/// and the input data for that day
#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
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
            });
        }
        _ => {
            return TokenStream::from(quote_spanned! {
                name.span() => compile_error!("Aoc day number out of range");
            })
        }
    };

    let AocTestAttributes {
        part1,
        part2,
    } = parse_macro_input!(attr as AocTestAttributes);

    let file = format!("../../data/day{}.txt", day_number);
    let day_index = LitInt::new(&(day_number - 1).to_string(), name.span());
    let fn_name = format_ident!("day{:02}", day_number);
    let part1_name = format_ident!("day{:02}a", day_number);
    let part2_name = format_ident!("day{:02}b", day_number);
    let hash = Punct::new('#', Spacing::Alone);

    let expanded = quote! {
        pub const solution: libaoc::Solution = libaoc::Solution {
            run: #name,
            file: include_str!(#file),
        };

        #ast

        #hash[cfg(test)]
        mod test {
            use crate::days::*;
            use anyhow::Result;

            #[test]
            fn #part1_name() -> Result<()> {
                let res = #fn_name::#fn_name(SOLUTIONS[#day_index].file.to_string())?;
                assert_eq!(res.part1, #part1);
                Ok(())
            }

            #[test]
            fn #part2_name() -> Result<()> {
                let res = #fn_name::#fn_name(SOLUTIONS[#day_index].file.to_string())?;
                assert_eq!(res.part2, #part2);
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
