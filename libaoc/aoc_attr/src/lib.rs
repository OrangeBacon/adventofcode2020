#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
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
/// and the input data for that day.  also generates the test suite for the day
#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    // re-name the original function, so it can be replaced with a getter
    let mut ast = parse_macro_input!(item as ItemFn);
    let provided_name = ast.sig.ident.clone();
    let new_name = format_ident!("{}_run", provided_name);
    ast.sig.ident = new_name.clone();

    // parse the file name into a day number
    let call_site = Span::call_site();
    let file = call_site.source_file().path();
    let file = file.file_name().unwrap().to_str().unwrap();
    let nums: String = file.chars().filter(|x| ('0'..='9').contains(x)).collect();
    let day_number = nums.parse::<u32>().unwrap();

    // parse the provided test results
    let AocTestAttributes { part1, part2 } = parse_macro_input!(attr as AocTestAttributes);

    let name = format!("Day {} - {}", day_number, provided_name);
    let file = format!("../../data/day{}.txt", day_number);
    let day_index = LitInt::new(&(day_number - 1).to_string(), provided_name.span());
    let fn_name = format_ident!("day{:02}", day_number);
    let part1_name = format_ident!("day{:02}a", day_number);
    let part2_name = format_ident!("day{:02}b", day_number);
    let hash = Punct::new('#', Spacing::Alone);

    let expanded = quote! {
        pub fn #provided_name() -> libaoc::Solution {
            libaoc::Solution {
                name: #name,
                run: #new_name,
                file: include_str!(#file),
            }
        }

        #ast

        #hash[cfg(test)]
        mod test {
            use crate::days::*;
            use anyhow::Result;

            #[test]
            fn #part1_name() -> Result<()> {
                let res = #fn_name::#provided_name()
                    .run(SOLUTIONS[#day_index]().file.to_string())?;
                assert_eq!(res.part1, #part1);
                Ok(())
            }

            #[test]
            fn #part2_name() -> Result<()> {
                let res = #fn_name::#provided_name()
                    .run(SOLUTIONS[#day_index]().file.to_string())?;
                assert_eq!(res.part2, #part2);
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
