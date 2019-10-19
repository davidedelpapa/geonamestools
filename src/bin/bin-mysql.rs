extern crate geonames;
extern crate structopt;

use std::error::Error;
use std::time::Instant;
use structopt::StructOpt;

use geonames::mysqlwriter::write_mysql;
use geonames::parser;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(short, long, default_value = "root")]
    user: String,
    #[structopt(short = "p", long, default_value = "")]
    password: String,
    #[structopt(short, long, default_value = "localhost")]
    host: String,
    #[structopt(short = "P", long, default_value = "3306")]
    port: i32,
    #[structopt(short, long, default_value = "root")]
    database: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let args = Cli::from_args();
    let mut parser = parser::Parser::new();
    parser.parse(args.input)?;
    write_mysql(
        parser,
        args.user,
        args.password,
        args.host,
        args.port,
        args.database,
    )?;
    let elapsed = now.elapsed();
    let elapsed_seconds =
        (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Elapsed seconds: {}", elapsed_seconds);
    Ok(())
}
