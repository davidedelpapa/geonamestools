use crate::parser::Parser;
use mysql::{error::Result, Pool};

pub fn write_mysql(input: Parser) -> Result<()> {
    let pool = Pool::new("mysql://root:passwd@172.17.0.2:3306/geodb")?;

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS cities (
            geonameid INT NOT NULL, 
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
        (),
    )?;
    // TODO: find a faster way!
    for result in input.into_iter() {
        pool.prep_exec(
            r"INSERT INTO cities(
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
                modification_date)
                VALUES
                    (:geonameid, :name, :asciiname, :alternatenames, :latitude,
                        :longitude, :feature_class, :feature_code, :country_code, :cc2, 
                        :admin1_code, :admin2_code, :admin3_code, :admin4_code, :population, 
                        :elevation, :dem, :timezone, :modification_date)",
            params! {
                "geonameid" => &result.geonameid,
                "name" => &result.name,
                "asciiname" => &result.asciiname,
                "alternatenames" => &result.alternatenames,
                "latitude" => &result.latitude,
                "longitude" => &result.longitude,
                "feature_class" => &result.feature_class.to_string(),
                "feature_code" => &result.feature_code,
                "country_code" => &result.country_code,
                "cc2" => &result.cc2,
                "admin1_code" => &result.admin1_code,
                "admin2_code" => &result.admin2_code,
                "admin3_code" => &result.admin3_code,
                "admin4_code" => &result.admin4_code,
                "population" => &result.population,
                "elevation" => &result.elevation,
                "dem" => &result.dem,
                "timezone" => &result.timezone,
                "modification_date" => &result.modification_date.to_string(),
            }
        )?;
    }
    Ok(())
}
