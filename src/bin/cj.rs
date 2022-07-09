use cj::extract_values;
use clap::{arg, command, ArgGroup};
use serde_json::Value;
use std::io::{self, Read};

fn main() {
    // Create application
    let matches = command!()
        // Add the json path arguments
        .arg(arg!(--"path" [PATH] "file path relative or absolute").short('p'))
        .arg(arg!([STDIN] "read from stdin"))
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::new("input")
                .required(false)
                .args(&["path", "STDIN"]),
        )
        .arg(
            arg!(--"filters" <FILTERS> "filters")
                .short('f')
                .required(false)
                .multiple_values(true),
        )
        .arg(
            arg!(--"compress" "compress JSON input")
                .short('c')
                .required(false),
        )
        .group(
            ArgGroup::new("print")
                .required(false)
                .args(&["filters", "compress"]),
        )
        .get_matches();

    let content = if let Some(j) = matches.value_of("path") {
        std::fs::read_to_string(j)
    } else {
        let mut input = vec![];
        io::stdin().lock().read_to_end(&mut input).unwrap();
        let input_str = String::from_utf8(input).unwrap_or_else(|op| {
            let e = format!("Error while reading stdin: {}", &op);
            println!("{}", e);
            e
        });
        Ok(input_str)
    };
    if content.is_err() {
        println!("Content err{:?}", content);
        std::process::exit(1)
    }

    let filters: Vec<&str> = match matches.values_of("filters") {
        Some(v) => v.collect(),
        None => vec![],
    };

    let compress_output = matches.is_present("compress");

    run(content.unwrap().as_str(), filters, compress_output);
}

fn run(content: &str, filters: Vec<&str>, compress: bool) {
    let val = match serde_json::from_str::<Value>(content) {
        Ok(val) => val,
        Err(e) => {
            println!("Invalid JSON: {}", e);
            std::process::exit(1)
        }
    };

    if filters.is_empty() {
        let json = if !compress {
            serde_json::ser::to_string_pretty(&val)
        } else {
            serde_json::ser::to_string(&val)
        };
        println!("{}", json.unwrap())
    }
    let values: Vec<Value> = extract_values(val, filters);
    values
        .iter()
        .map(|arr| {
            let mut val = serde_json::ser::to_string(&arr).unwrap();
            val.remove(0);
            val.pop();
            println!("{}", val);
        })
        .count();
}
