use chrono::prelude::*;
use holiday_gem_definitions::*;
use serde::Deserialize;
use std::collections::HashMap;

impl Holidays {
    pub fn default() -> Holidays {
        unimplemented!()
    }

    pub fn with_custom_definitions(definitions: HashMap<String, CountryFile>) -> Holidays {
        Holidays { definitions }
    }

    pub fn at_date<'a>(&'a self, date: &'a chrono::NaiveDate) -> QueryBuilder<'a> {
        QueryBuilder {
            date,
            holidays: self,
            country_code: None,

            region_code: None,
        }
    }
}

// TODO: RangeQueryBuilder
// TODO: country codes
pub struct QueryBuilder<'a> {
    date: &'a chrono::NaiveDate,
    holidays: &'a Holidays,
    region_code: Option<&'a str>,
    country_code: Option<&'a str>,
}

impl<'a> QueryBuilder<'a> {
    fn country_code(self, country_code: &'a str) -> Self {
        QueryBuilder {
            country_code: Some(country_code),
            ..self
        }
    }

    fn region_code(self, region_code: &'a str) -> Self {
        QueryBuilder {
            region_code: Some(region_code),
            ..self
        }
    }

    fn query(self) -> Option<&'a str> {
        self.holidays
            .definitions
            .months
            .lookup_date(&self.date)
            .map(|holiday| holiday.name.as_str())
    }
}
