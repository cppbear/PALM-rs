// Answer 0

#[test]
fn test_class_binary_query() {
    struct MockClassQuery(&'static str);
    
    impl ClassQuery<'static> for MockClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'static>> {
            Ok(CanonicalClassQuery::Binary(self.0))
        }
    }

    let query = MockClassQuery("SomeBinaryName");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_age_with_error() {
    struct MockClassQueryAge<'a>(&'a str);

    impl<'a> ClassQuery<'a> for MockClassQueryAge<'a> {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: "Age",
                property_value: self.0,
            })
        }
    }

    let query = MockClassQueryAge("InvalidValue");
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_by_value_script_extensions_with_error() {
    struct MockClassQueryScriptExtensions<'a>(&'a str);

    impl<'a> ClassQuery<'a> for MockClassQueryScriptExtensions<'a> {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: "Script_Extensions",
                property_value: self.0,
            })
        }
    }

    let query = MockClassQueryScriptExtensions("InvalidExtension");
    let result = class(query);
    assert!(result.is_err());
}

