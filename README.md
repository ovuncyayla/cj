# CJ
cj is a lightweight command-line tool to process JSON files blazingly fast :boom: (I had to say it because it is written in rust :stuck_out_tongue_winking_eye:).

## Features
It is in very early stage of development so there's lots of room to improve. (Feature requests and PR's are very welcome.) That being said, it can prettify / minify json input, process json-path expressions and prints out the results to stdout.

## Installation
Right now only way to install is using `cargo install` or building it from the source with `cargo build` command.

## Usage
```
cj 0.1.0

USAGE:
    cj [OPTIONS] [--] [STDIN]

ARGS:
    <STDIN>    read from stdin

OPTIONS:
    -c, --compress                compress JSON input
    -f, --filters <FILTERS>...    filters
    -h, --help                    Print help information
    -p, --path [<PATH>...]        file path relative or absolute
    -V, --version                 Print version information
```

## Examples
Assuming that you have `test.json` file in cwd with the following content, when no filter expression is given the default behaviour is pretty printing the json input. (Coloring support is coming soon...).
```
$ cat test.json | cj
# OR you can specify the absolute / relative file path with -p option
$ cj -p test.json
{
  "store": {
    "bicycle": {
      "color": "red",
      "price": 19.95
    },
    "book": [
      {
        "author": "Nigel Rees",
        "category": "reference",
        "price": 8.95,
        "title": "Sayings of the Century"
      },
      {
        "author": "Evelyn Waugh",
        "category": "fiction",
        "price": 12.99,
        "title": "Sword of Honour"
      },
      {
        "author": "Herman Melville",
        "category": "fiction",
        "isbn": "0-553-21311-3",
        "price": 8.99,
        "title": "Moby Dick"
      },
      {
        "author": "J. R. R. Tolkien",
        "category": "fiction",
        "isbn": "0-395-19395-8",
        "price": 22.99,
        "title": "The Lord of the Rings"
      }
    ]
  }
}
```
Filter books which costs lesser than 10:
```
$ cj -f $.store.book\[\?\(\@.price\ \<\ 10\)\].title 

# Outputs the following:
# "Sayings of the Century","Moby Dick"
```
You can also chain the filters:
```
$ cj -f '$.store.book[?(@.price < 10)].title' $..price

# Outputs the following:
# "Sayings of the Century","Moby Dick"
# 19.95,8.95,12.99,8.99,22.99
```

You can find more information about json path expressions [here](https://goessner.net/articles/JsonPath/ "here").

## Inspiration
Inspiration for this project comes from the beautiful tool [jq](https://github.com/stedolan/jq "jq"). You may check that out for a more feature-rich version of this project or in case you want a json processor tool wihout having to install rust/cargo on your system.

## LICENSE
[GNU General Public License v3.0 or later](https://spdx.org/licenses/GPL-3.0-or-later.html "GNU General Public License v3.0 or later")
