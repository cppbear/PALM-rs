fn canonical_value(
    vals: PropertyValues,
    normalized_value: &str,
) -> Option<&'static str> {
    ucd_util::canonical_property_value(vals, normalized_value)
}