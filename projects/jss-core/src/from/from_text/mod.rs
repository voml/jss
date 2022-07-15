use std::str::FromStr;

use jss_pest::{JssParser, Pair, Parser, Rule};

use crate::{JssError, JssSchema, Result};

impl FromStr for JssSchema {
    type Err = JssError;
    fn from_str(s: &str) -> Result<Self> {
        let mut ctx = ParseContext::default();
        ctx.parse(s)
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
        Self { errors: vec![] }
    }
}

impl ParseContext {
    pub fn parse(&mut self, input: &str) -> Result<JssSchema> {
        let parsed = JssParser::parse(Rule::program, input)?;
        let mut node = JssSchema::top();
        for pair in parsed {
            match pair.as_rule() {
                Rule::schema_statement => self.parse_schema_statement(pair, &mut node)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(node)
    }

    pub fn parse_schema_statement(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => node.set_name(pair.as_str()),
                Rule::object => self.parse_object(pair, node)?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
    pub fn parse_attitude_statement(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        let mut pairs = pairs.into_inner();
        let key = self.parse_key(pairs.next().unwrap())?;
        let value = pairs.next().unwrap();

        Ok(())
    }
}

impl ParseContext {
    pub fn parse_object(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::key => {
                    let key = self.parse_key(pair)?;
                }
                Rule::block => {
                    let block = self.parse_block(pair, node)?;
                }
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
    pub fn parse_block(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<String> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::attitude_statement => self.parse_attitude_statement(pair, node)?,
                _ => debug_cases!(pair),
            }
        }
        Err(JssError::unreachable())
    }
    pub fn parse_key(&mut self, pairs: Pair<Rule>) -> Result<String> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => return Ok(pair.as_str().to_string()),
                _ => debug_cases!(pair),
            }
        }
        Err(JssError::unreachable())
    }
}
