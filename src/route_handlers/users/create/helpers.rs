use regex::Regex;

use crate::errors::ApiError;

pub fn get_column_name_from_err_details(details: &str) -> Result<String, ApiError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Key \((?P<col_name>.*)\)=").unwrap();
    }

    let caps = RE.captures(details).ok_or_else(|| {
        ApiError::BadRequest("User with provided configuration already exists".to_string())
    })?;

    Ok(caps["col_name"].to_string())
}
