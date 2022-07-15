pub struct JssParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    EmptyLine,
    schema_statement,
    define_statement,
    _define,
    property_statement,
    _property,
    type_hint,
    COLON,
    data,
    Special,
    Byte,
    Cite,
    Number,
    SignedNumber,
    Decimal,
    DecimalBad,
    Integer,
    Exponent,
    Sign,
    String,
    StringEmpty,
    StringNormal,
    NS1,
    NS2,
    S1,
    S2,
    namespace,
    Key,
    SYMBOL,
    ExtraID,
    Dot,
    COMMENT,
    WHITESPACE,
    LineComment,
    MultiLineComment,
    SEPARATOR,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for JssParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::SEPARATOR(state).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::EmptyLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::EmptyLine(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLine(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EmptyLine, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.repeat(|state| self::WHITESPACE(state)).and_then(|state| self::NEWLINE(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn schema_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::schema_statement, |state| state.sequence(|state| state.match_string("schema").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::type_hint(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_statement, |state| state.sequence(|state| self::_define(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::SYMBOL(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringNormal(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn _define(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("def").or_else(|state| state.match_string("define"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn property_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::property_statement, |state| state.sequence(|state| self::_property(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::type_hint(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn _property(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("property")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_hint(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::type_hint, |state| state.sequence(|state| self::COLON(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COLON(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COLON, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::data, |state| self::Special(state).or_else(|state| state.restore_on_err(|state| self::Cite(state))).or_else(|state| self::Byte(state)).or_else(|state| self::Number(state)).or_else(|state| state.restore_on_err(|state| self::String(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Special(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Special, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Byte, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("0").and_then(|state| self::ASCII_ALPHA(state)).and_then(|state| self::ASCII_ALPHANUMERIC(state).or_else(|state| state.match_string("_")).or_else(|state| state.match_string("-"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Cite(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Cite, |state| state.sequence(|state| state.match_string("$").and_then(|state| self::namespace(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| self::Exponent(state).or_else(|state| self::SignedNumber(state)).and_then(|state| state.optional(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SignedNumber(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::SignedNumber, |state| state.sequence(|state| state.optional(|state| self::Sign(state)).and_then(|state| self::Decimal(state).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::DecimalBad, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))).or_else(|state| state.sequence(|state| self::Dot(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Exponent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Exponent, |state| state.sequence(|state| self::SignedNumber(state).and_then(|state| state.match_string("e").or_else(|state| state.match_string("E")).or_else(|state| state.match_string("**"))).and_then(|state| state.optional(|state| self::Sign(state))).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+").or_else(|state| state.match_string("-"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.optional(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::StringNormal(state)).or_else(|state| self::StringEmpty(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEmpty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEmpty, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::S1(state).and_then(|state| self::S1(state))).or_else(|state| state.sequence(|state| self::S2(state).and_then(|state| self::S2(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringNormal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::StringNormal, |state| state.restore_on_err(|state| state.sequence(|state| self::S1(state).and_then(|state| state.stack_push(|state| state.repeat(|state| self::S1(state)))).and_then(|state| state.sequence(|state| self::NS1(state).and_then(|state| state.repeat(|state| self::NS1(state))))).and_then(|state| self::POP(state)).and_then(|state| self::S1(state)))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::S2(state).and_then(|state| state.stack_push(|state| state.repeat(|state| self::S2(state)))).and_then(|state| state.sequence(|state| self::NS2(state).and_then(|state| state.repeat(|state| self::NS2(state))))).and_then(|state| self::POP(state)).and_then(|state| self::S2(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NS1(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NS1, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::S1(state).and_then(|state| self::PEEK(state)))).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::S1(state).and_then(|state| self::PEEK(state)))).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NS2(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NS2, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("\\u").and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state))).or_else(|state| state.sequence(|state| state.match_string("\\u").and_then(|state| state.match_string("{")).and_then(|state| state.sequence(|state| self::ASCII_HEX_DIGIT(state).or_else(|state| self::SPACE_SEPARATOR(state)).and_then(|state| state.repeat(|state| self::ASCII_HEX_DIGIT(state).or_else(|state| self::SPACE_SEPARATOR(state)))))).and_then(|state| state.match_string("}")))).or_else(|state| state.sequence(|state| state.match_string("\\").and_then(|state| self::ANY(state)))).or_else(|state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::S2(state).and_then(|state| self::PEEK(state)))).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::S2(state).and_then(|state| self::PEEK(state)))).and_then(|state| self::ANY(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S1(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::S1, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("'")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S2(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::S2, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\"")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn namespace(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::namespace, |state| state.sequence(|state| self::Key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Key(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Key(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::StringNormal(state)).or_else(|state| self::SYMBOL(state)).or_else(|state| self::SignedNumber(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).or_else(|state| self::ExtraID(state)).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state).or_else(|state| self::ExtraID(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ExtraID(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::ExtraID, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COMMENT, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::MultiLineComment(state).or_else(|state| self::LineComment(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::SPACE_SEPARATOR(state)).or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineComment, |state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::MultiLineComment, |state| state.sequence(|state| state.match_string("/*").and_then(|state| state.repeat(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("*/")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SEPARATOR, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",").or_else(|state| state.match_string(";"))))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHANUMERIC(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z')).or_else(|state| state.match_range('0'..'9'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::EmptyLine => rules::EmptyLine(state),
            Rule::schema_statement => rules::schema_statement(state),
            Rule::define_statement => rules::define_statement(state),
            Rule::_define => rules::_define(state),
            Rule::property_statement => rules::property_statement(state),
            Rule::_property => rules::_property(state),
            Rule::type_hint => rules::type_hint(state),
            Rule::COLON => rules::COLON(state),
            Rule::data => rules::data(state),
            Rule::Special => rules::Special(state),
            Rule::Byte => rules::Byte(state),
            Rule::Cite => rules::Cite(state),
            Rule::Number => rules::Number(state),
            Rule::SignedNumber => rules::SignedNumber(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Exponent => rules::Exponent(state),
            Rule::Sign => rules::Sign(state),
            Rule::String => rules::String(state),
            Rule::StringEmpty => rules::StringEmpty(state),
            Rule::StringNormal => rules::StringNormal(state),
            Rule::NS1 => rules::NS1(state),
            Rule::NS2 => rules::NS2(state),
            Rule::S1 => rules::S1(state),
            Rule::S2 => rules::S2(state),
            Rule::namespace => rules::namespace(state),
            Rule::Key => rules::Key(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::ExtraID => rules::ExtraID(state),
            Rule::Dot => rules::Dot(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::LineComment => rules::LineComment(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::SEPARATOR => rules::SEPARATOR(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
