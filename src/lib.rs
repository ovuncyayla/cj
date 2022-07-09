use std::str::FromStr;

use jsonpath_rust::{JsonPathFinder, JsonPathInst};
use serde_json::Value;

pub mod ui;

pub fn extract_values(json: Value, filters: Vec<&str>) -> Vec<Value> {
    let paths: Vec<JsonPathInst> = filters
        .iter()
        .map(|f| JsonPathInst::from_str(*f))
        .take_while(|p| p.is_ok())
        .map(|p| p.unwrap())
        .collect();

    let mut values: Vec<Value> = vec![];
    for path in paths {
        let finder = JsonPathFinder::new(Box::from(json.clone()), Box::from(path));
        values.push(finder.find());
    }
    values
}
