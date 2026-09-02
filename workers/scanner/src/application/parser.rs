use quick_xml::de::from_str;

use crate::{application::error::AppError, domain::result::NmapRun};

pub fn parse(xml: &str) -> Result<NmapRun, AppError> {
    Ok(from_str(xml)?)
}
