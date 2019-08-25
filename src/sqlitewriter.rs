use crate::parser::Parser;

use rusqlite::{Result, NO_PARAMS};

macro_rules! getopts {
    ($input:expr) => {
        $input.unwrap_or("-".to_string())
    };
}

macro_rules! getoptf {
    ($input:expr) => {
        $input.unwrap_or(0.0).to_string()
    };
}

macro_rules! getopti {
    ($input:expr) => {
        $input.unwrap_or(0).to_string()
    };
}

pub fn write_sqlite(input: Parser, output: std::path::PathBuf) -> Result<()> {
    let connection = rusqlite::Connection::open(output)?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS cities (
            geonameid INT, 
            name TEXT, 
            asciiname TEXT, 
            alternatenames TEXT,
            latitude REAL,
            longitude REAL,
            feature_class TEXT,
            feature_code TEXT,
            country_code TEXT,
            cc2 TEXT,
            admin1_code TEXT,
            admin2_code TEXT,
            admin3_code TEXT,
            admin4_code TEXT,
            population NUMERIC,
            elevation INT,
            dem REAL,
            timezone TEXT,
            modification_date TEXT
        )",
        NO_PARAMS,
    )?;
    for result in input.into_iter() {
        //let record: Geoname = result;
        connection.execute(
            "INSERT INTO cities (
                geonameid, 
                name, 
                asciiname, 
                alternatenames,
                latitude,
                longitude,
                feature_class,
                feature_code,
                country_code,
                cc2,
                admin1_code,
                admin2_code,
                admin3_code,
                admin4_code,
                population,
                elevation,
                dem,
                timezone,
                modification_date
            ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
            &[
                &result.geonameid.to_string(),
                &result.name,
                &getopts!(result.asciiname),
                &getopts!(result.alternatenames),
                &result.latitude.to_string(),
                &result.longitude.to_string(),
                &result.feature_class.to_string(),
                &result.feature_code,
                &getopts!(result.country_code),
                &getopts!(result.cc2),
                &getopts!(result.admin1_code),
                &getopts!(result.admin2_code),
                &getopts!(result.admin3_code),
                &getopts!(result.admin4_code),
                &getoptf!(result.population),
                &getopti!(result.elevation),
                &getoptf!(result.dem),
                &getopts!(result.timezone),
                &result.modification_date.to_string(),
            ],
        )?;
    }
    Ok(())
}
