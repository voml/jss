use crate::{JssError, JssSchema, Result};
use jss_pest::{JssParser, Parser, Rule};
use std::str::FromStr;

impl FromStr for JssSchema {
    type Err = JssError;
    fn from_str(s: &str) -> Result<Self> {
        ParseContext::parse(s)
    }
}

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

struct ParseContext {}

impl ParseContext {
    pub fn parse(input: &str) -> Result<JssSchema> {
        let parsed = JssParser::parse(Rule::program, input)?;
        for pair in parsed {
            match pair.as_rule() {
                _ => debug_cases!(pair),
            }
        }
        unreachable!()
    }
}
