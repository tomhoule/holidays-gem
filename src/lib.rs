use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CountryFile {
    pub months: Months,
    #[serde(default)]
    pub tests: Vec<Test>,
}

#[derive(Deserialize)]
pub struct Months {
    #[serde(rename = "0")]
    #[serde(default)]
    pub computed: Vec<ComputedHoliday>,
    #[serde(rename = "1")]
    #[serde(default)]
    pub january: Vec<Holiday>,
    #[serde(rename = "2")]
    #[serde(default)]
    pub february: Vec<Holiday>,
    #[serde(rename = "3")]
    #[serde(default)]
    pub march: Vec<Holiday>,
    #[serde(rename = "4")]
    #[serde(default)]
    pub april: Vec<Holiday>,
    #[serde(rename = "5")]
    #[serde(default)]
    pub may: Vec<Holiday>,
    #[serde(rename = "6")]
    #[serde(default)]
    pub june: Vec<Holiday>,
    #[serde(rename = "7")]
    #[serde(default)]
    pub july: Vec<Holiday>,
    #[serde(rename = "8")]
    #[serde(default)]
    pub august: Vec<Holiday>,
    #[serde(rename = "9")]
    #[serde(default)]
    pub september: Vec<Holiday>,
    #[serde(rename = "10")]
    #[serde(default)]
    pub october: Vec<Holiday>,
    #[serde(rename = "11")]
    #[serde(default)]
    pub november: Vec<Holiday>,
    #[serde(rename = "12")]
    #[serde(default)]
    pub december: Vec<Holiday>,
}

#[derive(Deserialize)]
pub struct Holiday {
    pub name: String,
    pub regions: Vec<String>,
    /// Day of the month.
    pub mday: Option<u8>,
}

/// https://github.com/holidays/definitions/blob/master/doc/SYNTAX.md#dates-defined-by-a-week-number-eg-first-monday-of-a-month
#[derive(Deserialize)]
#[repr(i8)]
pub enum Week {
    FirstWeek = 1,
    SecondWeek = 2,
    ThirdWeek = 3,
    FourthWeek = 4,
    LastWeek = -1,
}

/// https://github.com/holidays/definitions/blob/master/doc/SYNTAX.md#dates-defined-by-a-week-number-eg-first-monday-of-a-month
#[derive(Deserialize)]
#[repr(u8)]
pub enum WeekDay {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[derive(Deserialize)]
pub struct ComputedHoliday {
    pub name: String,
    pub regions: Vec<String>,
    pub function: String,
    pub function_modifier: Option<i16>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Deserialize)]
pub struct Test {
    pub given: Given,
    pub expect: Expectation,
}

#[derive(Deserialize)]
pub struct Expectation {
    pub name: Option<String>,
    pub holiday: Option<bool>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum OneOrList<T> {
    One(T),
    List(Vec<T>),
}

#[derive(Deserialize)]
pub struct Given {
    pub date: OneOrList<String>,
    pub regions: Vec<String>,
    pub options: Option<OneOrList<String>>,
}

/// The shape of the data inside the index.yaml file.
#[derive(Deserialize)]
pub struct Index {
    pub defs: HashMap<String, Vec<std::path::PathBuf>>,
}

pub struct Holidays {
    definitions: CountryFile,
}

impl Holidays {}
