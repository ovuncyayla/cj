use clap::{arg, command, ArgGroup};
use serde_json::Value;
use std::io::{self, Read};

fn main() {
    // Create application
    let matches = command!()
        // Add the json path arguments
        .arg(arg!(--"path" [PATH] "file path relative or absolute").short('p'))
        // .arg(arg!(--"url" <URL> "fetch from url").short('u'))
        .arg(arg!([JSON] "read from stdin"))
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::new("input")
                .required(false)
                .args(&["path", "JSON"]),
        )
        .get_matches();

    let content = if let Some(j) = matches.value_of("path") {
        std::fs::read_to_string(j)
    } else if let Some(j) = matches.value_of("JSON") {
        Ok(j.to_string())
    } else {
        let mut buff = String::new();
        let mut stdin = io::stdin();
        stdin
            .read_to_string(&mut buff).unwrap();
        Ok(buff)
    };
    let val : Value = serde_json::from_str(content.unwrap().as_str()).unwrap();
    let json = serde_json::ser::to_string_pretty(&val);
    println!("{}", json.unwrap());
}
