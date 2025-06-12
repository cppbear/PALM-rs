pub fn class<'a>(query: ClassQuery<'a>) -> Result<hir::ClassUnicode> {
    use self::CanonicalClassQuery::*;

    match query.canonicalize()? {
        Binary(name) => {
            property_set(property_bool::BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyNotFound)
        }
        GeneralCategory("Any") => {
            Ok(hir_class(&[('\0', '\u{10FFFF}')]))
        }
        GeneralCategory("Assigned") => {
            let mut cls =
                property_set(general_category::BY_NAME, "Unassigned")
                    .map(hir_class)
                    .ok_or(Error::PropertyNotFound)?;
            cls.negate();
            Ok(cls)
        }
        GeneralCategory("ASCII") => {
            Ok(hir_class(&[('\0', '\x7F')]))
        }
        GeneralCategory(name) => {
            property_set(general_category::BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
        Script(name) => {
            property_set(script::BY_NAME, name)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
        ByValue { property_name: "Age", property_value } => {
            let mut class = hir::ClassUnicode::empty();
            for set in ages(property_value)? {
                class.union(&hir_class(set));
            }
            Ok(class)
        }
        ByValue { property_name: "Script_Extensions", property_value } => {
            property_set(script_extension::BY_NAME, property_value)
                .map(hir_class)
                .ok_or(Error::PropertyValueNotFound)
        }
        _ => {
            // What else should we support?
            Err(Error::PropertyNotFound)
        }
    }
}