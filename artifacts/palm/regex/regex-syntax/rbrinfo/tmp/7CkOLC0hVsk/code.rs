fn canonical_prop(normalized_name: &str) -> Option<&'static str> {
    ucd_util::canonical_property_name(PROPERTY_NAMES, normalized_name)
}