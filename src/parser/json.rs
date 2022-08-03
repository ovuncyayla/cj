// Based on https://pest.rs/book/examples/json.html
use pest;
use pest::error::{Error};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar/json.pest"]
struct JSONParser;

#[derive(Debug)]
pub enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn parse_value(pair: Pair<Rule>) -> JSONValue {
    match pair.as_rule() {
        Rule::object => JSONValue::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();
                    let name = inner_rules
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str();
                    let value = parse_value(inner_rules.next().unwrap());
                    (name, value)
                })
                .collect(),
        ),
        Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
        Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
        Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
        Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
        Rule::null => JSONValue::Null,
        Rule::json
        | Rule::EOI
        | Rule::pair
        | Rule::value
        | Rule::inner
        | Rule::char
        | Rule::WHITESPACE => unreachable!(),
    }
}

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub message: String
}

impl ParserError {
    pub fn new(message: String) -> ParserError {
        ParserError { message }
    }
}


impl From<Error<Rule>> for ParserError {

    fn from(err: Error<Rule>) -> Self {
        let message = match err.line_col {
            pest::error::LineColLocation::Pos(p) => {
                format!("Position -  Line: {}, Col: {}", p.0, p.1)
            },
            pest::error::LineColLocation::Span(s1, s2) => {
                format!("Position: {} - {}, {}, {}", s1.0, s1.1, s2.0, s2.1)
            },
        };
        ParserError { message }
    }
}

pub fn from_str(str: &str) -> Result<JSONValue, ParserError> {
    match JSONParser::parse(Rule::json, str) {
        Ok(mut result) => {
            let json = match result.next() {
                Some(content) => parse_value(content),
                None => JSONValue::String(""),
            };
            Ok(json)
        }
        Err(e) => Err(ParserError::from(e))
    }
}

pub fn serialize(val: &JSONValue) -> String {
    use JSONValue::{Array, Boolean, Null, Number, Object, String};

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, serialize(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => format!("null"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_object() {
        let unparsed_file = r#"
        {
            "nesting": { "inner object": {} },
            "an array": [1.5, true, null, 1e-6],
            "string with escaped double quotes" : "\"quick brown foxes\""
        }
    "#;
        let json: JSONValue = from_str(&unparsed_file).unwrap();
        assert_eq!("{\"nesting\":{\"inner object\":{}},\"an array\":[1.5,true,null,0.000001],\"string with escaped double quotes\":\"\\\"quick brown foxes\\\"\"}"
            , serialize(&json));
    }

    #[test]
    fn serialize_int() {
        let unparsed_file = "1";
        let json: JSONValue = from_str(&unparsed_file).unwrap();
        assert_eq!("1", serialize(&json));
    }

    #[test]
    fn serialize_string() {
        let unparsed_file = "\"asdasd\"";
        let json: JSONValue = from_str(&unparsed_file).unwrap();
        assert_eq!("\"asdasd\"", serialize(&json));
    }

    #[test]
    fn serialize_invalid_json() {
        let unparsed_file = "Invalid";
        let json: ParserError = from_str(&unparsed_file).expect_err("QWQEQWE");
        assert_eq!(ParserError::new("Position -  Line: 1, Col: 1".to_string()), json);
    }

}
