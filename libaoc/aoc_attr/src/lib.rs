#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Ident, ItemFn, LitInt, LitStr, Token,
};

struct AocTestAttributes {
    part1: LitStr,
    part2: LitStr,
    file: Option<LitStr>,
}

impl Parse for AocTestAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let part1: LitStr = input.parse()?;
        input.parse::<Token!(,)>()?;
        let part2: LitStr = input.parse()?;
        let file = match input.parse::<Token!(,)>() {
            Ok(_) => Some(input.parse()?),
            Err(_) => None,
        };
        Ok(Self { part1, part2, file })
    }
}

fn global_impl(attr: AocTestAttributes, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    let provided_name = ast.sig.ident.clone();

    // parse the file name into a day number
    let call_site = Span::call_site();
    let file = call_site.source_file().path();
    let file = file.file_name().unwrap().to_str().unwrap();
    let nums: String = file.chars().filter(|x| ('0'..='9').contains(x)).collect();
    let day_number = nums.parse::<u32>().unwrap();

    let AocTestAttributes { part1, part2, file } = attr;

    let name = provided_name.to_string();

    // use absolute path otherwise rustc breaks
    let file = if let Some(file) = file {
        quote! { #file }
    } else {
        let file_name = format!(
            "{}/data/day{}.txt",
            std::env::current_dir().unwrap().to_str().unwrap(),
            day_number
        );
        quote! {include_str!(#file_name)}
    };

    let part1_name = format_ident!("day{:02}a", day_number);
    let part2_name = format_ident!("day{:02}b", day_number);
    let day_number = LitInt::new(&day_number.to_string(), provided_name.span());

    let append = Ident::new(
        &provided_name.to_string().to_uppercase(),
        provided_name.span(),
    );

    let expanded = quote! {
        #[linkme::distributed_slice(crate::SOLUTIONS)]
        static #append: libaoc::Solution = libaoc::Solution {
            number: #day_number,
            name: #name,
            run: #provided_name,
            takes_file_name: false,
            cleanup_fn: None,
        };

        #[linkme::distributed_slice(crate::FILES)]
        static FILE: libaoc::AocFile = libaoc::AocFile {
            number: #day_number,
            data: #file,
        };

        #ast

        #[aoc(
            cleanup_fn = fn bench_cleanup() {
                criterion::Criterion::default().final_summary();
            }
        )]
        pub fn bench(timer: &mut libaoc::Timer, input: &str) -> Result<libaoc::AocResult> {
            use criterion::*;
            println!("test");
            let solution = libaoc::Solution::get(&*crate::SOLUTIONS, #day_number, "solve")?;
            let file = libaoc::AocFile::get(&*crate::FILES, #day_number)?;
            let mut c = Criterion::default();

            c.bench_with_input(BenchmarkId::new("aoc", #day_number), &file, |b, &s| {
                b.iter(|| {
                    let mut timer = Timer::new();
                    let _ = solution.run(&mut timer, &s);
                });
            });

            Ok(libaoc::AocResult::default())
        }

        #[cfg(test)]
        mod test {
            use crate::days::*;
            use anyhow::Result;
            use libaoc::Timer;

            #[test]
            fn #part1_name() -> Result<()> {
                let solution = libaoc::Solution::get(&*crate::SOLUTIONS, #day_number, "solve")?;
                let file = libaoc::AocFile::get(&*crate::FILES, #day_number)?;
                let mut timer = Timer::new();
                let res = solution.run(&mut timer, file)?;
                assert_eq!(res.results[0].1, #part1);
                Ok(())
            }

            #[test]
            fn #part2_name() -> Result<()> {
                let solution = libaoc::Solution::get(&*crate::SOLUTIONS, #day_number, "solve")?;
                let file = libaoc::AocFile::get(&*crate::FILES, #day_number)?;
                let mut timer = Timer::new();
                let res = solution.run(&mut timer, file)?;
                assert_eq!(res.results[1].1, #part2);
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

struct AocGeneralAttributes {
    cleanup_fn: Option<ItemFn>,
}

impl Parse for AocGeneralAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(ident) = input.parse::<Ident>() {
            assert_eq!(ident.to_string(), "cleanup_fn");
            input.parse::<Token!(=)>()?;
            let func = input.parse::<ItemFn>()?;
            Ok(AocGeneralAttributes {
                cleanup_fn: Some(func),
            })
        } else {
            Ok(AocGeneralAttributes { cleanup_fn: None })
        }
    }
}

fn local_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    let attr = parse_macro_input!(attr as AocGeneralAttributes);
    let provided_name = ast.sig.ident.clone();

    let append = Ident::new(
        &provided_name.to_string().to_uppercase(),
        provided_name.span(),
    );

    let call_site = Span::call_site();
    let file = call_site.source_file().path();
    let file = file.file_name().unwrap().to_str().unwrap();
    let nums: String = file.chars().filter(|x| ('0'..='9').contains(x)).collect();
    let day_number = nums.parse::<u32>().unwrap();
    let day_number = LitInt::new(&day_number.to_string(), provided_name.span());

    let name = provided_name.to_string();

    let (cleanup_fn, cleanup_ast) = if let Some(attr) = attr.cleanup_fn {
        let name = attr.sig.ident.clone();
        (quote! { Some(#name) }, quote! { #attr })
    } else {
        (quote! { None }, quote! {})
    };

    let expanded = quote! {
        #[linkme::distributed_slice(crate::SOLUTIONS)]
        static #append: libaoc::Solution = libaoc::Solution {
            number: #day_number,
            name: #name,
            run: #provided_name,
            takes_file_name: false,
            cleanup_fn: #cleanup_fn,
        };
        #cleanup_ast
        #ast
    };

    TokenStream::from(expanded)
}

/// annotates a function with a solution struct containing the function
/// and the input data for that day.  also generates the test suite for the day
#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the provided test results
    let res = syn::parse(attr.clone());

    match res {
        Ok(attr) => global_impl(attr, item),
        Err(_) => local_impl(attr, item),
    }
}
