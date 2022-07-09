use cj::ui::{app::AppResult, run::run};
use clap::{arg, command, ArgGroup};
use std::io::{self, Read};

fn main() -> AppResult<()> {
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
    run(content.unwrap())
}
