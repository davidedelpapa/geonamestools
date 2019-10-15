extern crate geonames;
extern crate structopt;

use std::error::Error;
use std::time::Instant;
use structopt::StructOpt;

use geonames::parser;
use geonames::mysqlwriter::write_mysql;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let args = Cli::from_args();
    let mut parser = parser::Parser::new();
    parser.parse(args.input)?;
    write_mysql(parser)?;
    let elapsed =  now.elapsed();
    let elapsed_seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Elapsed seconds: {}", elapsed_seconds);
    Ok(())
}
