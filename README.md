# GeoNamesTools

Library and CLI tool to use data from [GeoNames.org](http://www.geonames.org/)

## CLI (standard)

For now it can process in a simple way the cities data(for example cities-1000) and transfer data to a sqlite DB.

More features to come!

Example Usage:

``` sh
cargo run ./test_data/test_cities.txt test.db 
```

The line above transfers the data in the test_file present in *test_cities.txt* to the database  *test.db* (creating it, if it does not exist).

### Beware of times!

Right now, on my generic laptop, it took 11 minutes to parse all of cities1000!

## CLI (MySQL)

It is a porting to MySQL of the main CLI

Example Usage:

``` sh
cargo run --bin gntools-mysql -- -u root -p passwd -h 172.17.0.2 -P 3306 -d geodb ./test_data/test_cities.txt
```
(I'm using docker!)

## Library

For now it has the two components

``` Rust
geonames::parser;
geonames::sqlitewriter::write_sqlite;
```

Example:
``` Rust
extern crate geonames;
use geonames::parser;
use geonames::sqlitewriter::write_sqlite;

let input: std::path::PathBuf = "./test_data/test_cities.txt";
let output: std::path::PathBuf = "output.db";

let mut parser = parser::Parser::new(); // Creates the new parser
parser.parse(input).expect("Failure to parse"); // Feeds the test data to the parser
write_sqlite(parser, output).expect("Failure to write"); // Writes the read data to the SQLite DB
```

## License
MIT or Apache-2.0
