use std::fmt::{self, Display, Formatter};

pub struct LookupResult(Result<&'static &'static str, ConversionError>);

impl Display for LookupResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match &self.0 {
            Ok(s) => format!("{}", s),
            Err(e) => format!("{}", e),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversionError {
    Unimplemented(usize),
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unimplemented(code) => write!(f, "code '{}' is not implemented", code),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/cct00.rs"));
include!(concat!(env!("OUT_DIR"), "/cct11.rs"));

/// Implements "Code Table 1.1: GRIB Local Tables Version Number"
pub const CODE_TABLE_1_1: &'static [&'static str] = &[
    "Local tables not used. Only table entries and templates from the current Master table are valid.",
];

/// Implements "Code Table 1.2: Significance of Reference Time"
pub const CODE_TABLE_1_2: &'static [&'static str] = &[
    "Analysis",
    "Start of forecast",
    "Verifying time of forecast",
    "Observation time",
];

/// Implements "Code Table 1.3: Production status of data"
pub const CODE_TABLE_1_3: &'static [&'static str] = &[
    "Operational products",
    "Operational test products",
    "Research products",
    "Re-analysis products",
];

/// Implements "Code Table 1.4: Type of data"
pub const CODE_TABLE_1_4: &'static [&'static str] = &[
    "Analysis products",
    "Forecast products",
    "Analysis and forecast products",
    "Control forecast products",
    "Perturbed forecast products",
    "Control and perturbed forecast products",
    "Processed satellite observations",
    "Processed radar observations",
];

pub fn lookup_table(table: &'static [&'static str], code: usize) -> LookupResult {
    let result = table.get(code).ok_or(ConversionError::Unimplemented(code));
    LookupResult(result)
}
