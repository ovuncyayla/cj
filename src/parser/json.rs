// Based on https://pest.rs/book/examples/json.html

use pest;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar/json.pest"]
struct JSONParser;

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

pub fn from_str(str: &str) -> Result<JSONValue, Error<&str>> {
     match JSONParser::parse(Rule::json, str) {
        Ok(mut result) => {
            let json = match result.next() {
                Some(content) => parse_value(content),
                None => JSONValue::String("")
            };
            Ok(json)
        },
        Err(e) => {
            dbg!(e.clone());
            todo!()
        }
    }
}

pub fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
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
    fn serialize() {
        let unparsed_file = r#"
        {
            "nesting": { "inner object": {} },
            "an array": [1.5, true, null, 1e-6],
            "string with escaped double quotes" : "\"quick brown foxes\""
        }
    "#;
        let json: JSONValue = from_str(&unparsed_file).unwrap();
        assert_eq!("{\"nesting\":{\"inner object\":{}},\"an array\":[1.5,true,null,0.000001],\"string with escaped double quotes\":\"\\\"quick brown foxes\\\"\"}"
            , serialize_jsonvalue(&json));
    }

}
