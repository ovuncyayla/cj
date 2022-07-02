use clap::{arg, command, ArgGroup};
use jsonpath_rust::{JsonPathFinder, JsonPathInst};
use serde_json::Value;
use std::{
    io::{self, Read},
    str::FromStr,
};

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
        return;
    }

    let filters: Vec<&str> = match matches.values_of("filters") {
        Some(v) => v.collect(),
        None => vec![],
    };

    let compress_output = matches.is_present("compress");

    run(content.unwrap().as_str(), filters, compress_output);
}

fn run(content: &str, filters: Vec<&str>, compress: bool) {
    match serde_json::from_str::<Value>(content) {
        Ok(val) => {
            if filters.is_empty() {
                let json = if !compress {
                    serde_json::ser::to_string_pretty(&val)
                } else {
                    serde_json::ser::to_string(&val)
                };
                println!("{}", json.unwrap())
            }
            extract_values(val, filters)
        }
        Err(e) => println!("Invalid JSON: {}", e),
    }
}

fn extract_values(json: Value, filters: Vec<&str>) {
    let paths: Vec<JsonPathInst> = filters
        .iter()
        .map(|f| JsonPathInst::from_str(*f))
        .take_while(|p| p.is_ok())
        .map(|p| p.unwrap())
        .collect();

    for path in paths {
        let finder = JsonPathFinder::new(Box::from(json.clone()), Box::from(path));

        match &finder.find() {
            Value::Array(p) => {
                let mut val = serde_json::ser::to_string(&p).unwrap().to_string();
                val.remove(0);
                val.pop();
                println!("{}", val);
            }
            _ => unreachable!(),
        }
    }
}
