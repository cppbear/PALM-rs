fn property_values(
    canonical_property_name: &'static str,
) -> Option<PropertyValues>
{
    ucd_util::property_values(PROPERTY_VALUES, canonical_property_name)
}