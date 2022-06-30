use clap::{arg, command, ArgGroup};
use serde_json::Value;
use std::io::{self, Read};

fn main() {
    // Create application
    let matches = command!()
        // Add the json path arguments
        .arg(arg!(--"path" [PATH] "file path relative or absolute").short('p'))
        // .arg(arg!(--"url" <URL> "fetch from url").short('u'))
        .arg(arg!([STDIN] "read from stdin"))
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::new("input")
                .required(false)
                .args(&["path", "STDIN"]),
        )
        .get_matches();

    let content = if let Some(j) = matches.value_of("path") {
        std::fs::read_to_string(j)
    } else if let Some(j) = matches.value_of("STDIN") {
        Ok(j.to_string())
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
        return;
    }
    if let Ok(val) = serde_json::from_str::<Value>(content.unwrap().as_str()) {
        let json = serde_json::ser::to_string_pretty(&val);
        println!("{}", json.unwrap());
    }
}
