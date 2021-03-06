#[macro_use]
extern crate quote;
extern crate pest_generator;
extern crate proc_macro;
use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
#[ignore]
pub fn gen_parser() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./jss.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./src/jss.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct JssParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct JssParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}
