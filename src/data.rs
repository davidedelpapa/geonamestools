use chrono::NaiveDate;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geoname {
    pub geonameid: i64,
    pub name: String,
    pub asciiname: Option<String>,
    pub alternatenames: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub feature_class: FeatureClass,
    pub feature_code: String,
    pub country_code: Option<String>,
    pub cc2: Option<String>,
    pub admin1_code: Option<String>,
    pub admin2_code: Option<String>,
    pub admin3_code: Option<String>,
    pub admin4_code: Option<String>,
    pub population: Option<f64>,
    pub elevation: Option<i32>,
    pub dem: Option<f64>,
    pub timezone: Option<String>,
    #[serde(with = "geonames_date_format")]
    pub modification_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum FeatureClass {
    A, // country, state, region,...
    H, // stream, lake, ...
    L, // parks,area, ...
    P, // city, village,...
    R, // road, railroad
    S, // spot, building, farm
    T, // mountain,hill,rock,...
    U, // undersea
    V, // forest,heath,...
}
impl FeatureClass {
    fn to_string(&self) -> String {
        match *self {
            FeatureClass::A => "A".to_string(),
            FeatureClass::H => "H".to_string(),
            FeatureClass::L => "L".to_string(),
            FeatureClass::P => "P".to_string(),
            FeatureClass::R => "R".to_string(),
            FeatureClass::S => "S".to_string(),
            FeatureClass::T => "T".to_string(),
            FeatureClass::U => "U".to_string(),
            FeatureClass::V => "V".to_string(),
        }
    }
}
impl fmt::Display for FeatureClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

mod geonames_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
