use crate::data::Geoname;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Parser {
    inner: Vec<Geoname>,
}
impl Parser {
    pub fn new() -> Self {
        Parser { inner: Vec::new() }
    }

    pub fn parse(&mut self, path: std::path::PathBuf) -> Result<&Parser, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(true)
            .from_path(path)?;
        for result in rdr.deserialize() {
            let record: Geoname = result?;
            self.inner.push(record);
        }
        Ok(self)
    }
}

impl IntoIterator for Parser {
    type Item = Geoname;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
