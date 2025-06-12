// Answer 0

#[test]
fn test_canonicalize_by_value() {
    struct ClassQuery {
        property_name: String,
        property_value: String,
    }
    
    struct CanonicalClassQuery {
        property_name: String,
        property_value: String,
    }
    
    impl ClassQuery {
        fn canonicalize(&self) -> Result<CanonicalClassQuery, String> {
            let property_name = &self.property_name;
            let property_value = &self.property_value;

            let canon_name = match canonical_prop(property_name) {
                None => return Err("Property not found".to_string()),
                Some(canon_name) => canon_name.to_string(),
            };

            // Ensure canon_name is neither "General_Category" nor "Script"
            if canon_name == "General_Category" || canon_name == "Script" {
                return Err("Invalid canon_name".to_string());
            }

            let vals = match property_values(&canon_name) {
                None => return Err("Property value not found".to_string()),
                Some(vals) => vals,
            };

            let canon_val = match canonical_value(vals, property_value) {
                None => return Err("Property value not found".to_string()),
                Some(canon_val) => canon_val.to_string(),
            };

            Ok(CanonicalClassQuery {
                property_name: canon_name,
                property_value: canon_val,
            })
        }
    }

    fn canonical_prop(name: &str) -> Option<&str> {
        // Simulated lookup for the test
        match name {
            "Some_Property" => Some("Some_Canonical"),
            _ => None,
        }
    }

    fn property_values(canon_name: &str) -> Option<Vec<String>> {
        // Simulated property values for the test
        match canon_name {
            "Some_Canonical" => Some(vec!["value1".to_string(), "value2".to_string()]),
            _ => None,
        }
    }

    fn canonical_value(vals: Vec<String>, value: &str) -> Option<&str> {
        // Simulated canonical value determination
        if vals.contains(&value.to_string()) {
            Some("canonical_value1")
        } else {
            None
        }
    }

    let query = ClassQuery {
        property_name: "Some_Property".to_string(),
        property_value: "value1".to_string(),
    };

    let result = query.canonicalize();
    assert!(result.is_ok());

    let canonical_class_query = result.unwrap();
    assert_eq!(canonical_class_query.property_name, "Some_Canonical");
    assert_eq!(canonical_class_query.property_value, "canonical_value1");
}

