use std::str::FromStr;

use jss_pest::{JssParser, Pair, Parser, Rule};

use crate::{JssError, JssSchema, Result};

impl FromStr for JssSchema {
    type Err = JssError;
    fn from_str(s: &str) -> Result<Self> {
        let mut ctx = ParseContext::default();
        ctx.parse(s)?;
        Ok(JssSchema::from(ctx))
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

struct ParseContext {
    schema: JssSchema,
    errors: Vec<JssError>,
}

impl Default for ParseContext {
    fn default() -> Self {
        Self { schema: Default::default(), errors: vec![] }
    }
}

impl From<ParseContext> for JssSchema {
    fn from(ctx: ParseContext) -> Self {
        ctx.schema
    }
}

impl ParseContext {
    pub fn parse(&mut self, input: &str) -> Result<()> {
        let parsed = JssParser::parse(Rule::program, input)?;
        for pair in parsed {
            match pair.as_rule() {
                Rule::schema_statement => self.parse_schema_statement(pair)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }

    pub fn parse_schema_statement(&mut self, pairs: Pair<Rule>) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => self.schema.set_name(pair.as_str()),
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
}
