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
        let stdin = io::stdin();
        let mut buff2 = vec![];
        stdin.lock().read_to_end(&mut buff2).unwrap();
        let buff = String::from_utf8(buff2).unwrap_or_else(|op| {
            let e = format!("Error while reading stdin: {}", &op);
            println!("{}", e);
            e
        });
        Ok(buff)
    };
    if content.is_err() {
        return;
    }
    if let Ok(val) = serde_json::from_str::<Value>(content.unwrap().as_str()) {
        let json = serde_json::ser::to_string_pretty(&val);
        println!("{}", json.unwrap());
    }
}
