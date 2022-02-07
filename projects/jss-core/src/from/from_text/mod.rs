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
    errors: Vec<JssError>,
}

impl Default for ParseContext {
    fn default() -> Self {
        Self { top_level: JssSchema::top(), errors: vec![] }
    }
}

impl From<ParseContext> for JssSchema {
    fn from(ctx: ParseContext) -> Self {
        ctx.top_level
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
                Rule::SYMBOL => self.top_level.set_name(pair.as_str()),
                Rule::object => {
                    let ptr = &mut self.top_level;
                    self.parse_object(pair, ptr)?
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl ParseContext {
    pub fn parse_object(&self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::key => {
                    let key = self.parse_key(pair)?;
                }
                Rule::block => {
                    let block = self.parse_block(pair)?;
                }
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
    pub fn parse_block(&self, pairs: Pair<Rule>) -> Result<String> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => debug_cases!(pair),
            }
        }
        Err(JssError::unreachable())
    }
    pub fn parse_key(&self, pairs: Pair<Rule>) -> Result<String> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => return Ok(pair.as_str().to_string()),
                _ => debug_cases!(pair),
            }
        }
        Err(JssError::unreachable())
    }
}
