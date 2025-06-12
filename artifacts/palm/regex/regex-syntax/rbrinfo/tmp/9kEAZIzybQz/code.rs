fn canonical_script(normalized_value: &str) -> Option<&'static str> {
    let scripts = property_values("Script").unwrap();
    canonical_value(scripts, normalized_value)
}