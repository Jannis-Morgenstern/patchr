use regex::Regex;

pub fn is_valid_package_name(value: &str) -> Result<(), String> {
    let regex = Regex::new(r"^(@[a-z0-9-~][a-z0-9-._~]*/)?[a-z0-9-~][a-z0-9-._~]*$").unwrap();
    if regex.is_match(value) {
        return Ok(());
    }
    Err(String::from(format!(
        "`{}` is not a valid name for a package.",
        value
    )))
}
