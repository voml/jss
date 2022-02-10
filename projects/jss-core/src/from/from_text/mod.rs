use std::str::FromStr;
use json_value::Number;

use jss_pest::{JssParser, Pair, Parser, Rule};

use crate::{JssError, JssSchema, JssType, JssValue, Result};

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
                Rule::EOI => continue,
                Rule::schema_statement => self.parse_schema_statement(pair, &mut node)?,
                Rule::property_statement => {
                    let property = self.parse_property_statement(pair)?;
                    let name = property.get_name().to_string();
                    node.insert_property(name, property);
                }
                _ => debug_cases!(pair),
            }
        }
        Ok(node)
    }

    pub fn parse_schema_statement(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => node.set_name(pair.as_str()),
                Rule::block => self.parse_block(pair, node)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
    pub fn parse_property_statement(&mut self, pairs: Pair<Rule>) -> Result<JssSchema> {
        let mut out = JssSchema::property();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => out.set_name(pair.as_str()),
                Rule::block => self.parse_block(pair, &mut out)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(out)
    }
    pub fn parse_attitude_statement(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        let mut pairs = pairs.into_inner();
        let key = self.parse_key(pairs.next().unwrap())?;
        let value = self.parse_data(pairs.next().unwrap())?;
        node.attribute.insert(key, value);
        Ok(())
    }
}

impl ParseContext {
    pub fn parse_object(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::attitude_statement => self.parse_attitude_statement(pair, node)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
    pub fn parse_block(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::key => node.set_type(self.parse_key(pair)?),
                Rule::object => self.parse_object(pair, node)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
    pub fn parse_key(&mut self, pairs: Pair<Rule>) -> Result<String> {
        let pair = pairs.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::SYMBOL => return Ok(pair.as_str().to_string()),
            Rule::STRING_INLINE => return self.parse_string_inline(pair),
            _ => debug_cases!(pair),
        }
    }
    pub fn parse_data(&mut self, pairs: Pair<Rule>) -> Result<JssValue> {
        let pair = pairs.into_inner().next().unwrap();
        let value = match pair.as_rule() {
            Rule::URL => JssValue::Url(pair.as_str().to_string()),
            Rule::array => self.parse_array(pair)?,
            Rule::STRING_INLINE => JssValue::String(self.parse_string_inline(pair)?),
            Rule::Number => JssValue::Number(Number::from())
            _ => debug_cases!(pair),
        };
        Ok(value)
    }
    pub fn parse_array(&mut self, pairs: Pair<Rule>) -> Result<JssValue> {
        let mut out = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::data => out.push(self.parse_data(pair)?),
                _ => debug_cases!(pair),
            }
        }
        return Ok(JssValue::Array(out));
    }
    pub fn parse_string_inline(&mut self, pairs: Pair<Rule>) -> Result<String> {
        let mut out = String::with_capacity(pairs.as_str().len());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::NS2 => out.push_str(pair.as_str()),
                _ => debug_cases!(pair),
            }
        }
        return Ok(out);
    }
}
