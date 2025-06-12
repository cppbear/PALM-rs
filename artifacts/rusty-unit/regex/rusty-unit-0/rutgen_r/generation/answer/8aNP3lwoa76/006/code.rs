// Answer 0

#[test]
fn test_class_binary_property() {
    struct FakeClassQuery;

    impl ClassQuery<'_> for FakeClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'_>> {
            Ok(CanonicalClassQuery::Binary("SomePropertyName"))
        }
    }

    let query = FakeClassQuery;
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_age_valid() {
    struct FakeClassQuery;

    impl ClassQuery<'_> for FakeClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'_>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: "Age",
                property_value: "22",
            })
        }
    }

    let query = FakeClassQuery;
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_age_invalid() {
    struct FakeClassQuery;

    impl ClassQuery<'_> for FakeClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'_>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: "Age",
                property_value: "InvalidAge",
            })
        }
    }

    let query = FakeClassQuery;
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_by_value_script_extensions_valid() {
    struct FakeClassQuery;

    impl ClassQuery<'_> for FakeClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'_>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: "Script_Extensions",
                property_value: "Latn",
            })
        }
    }

    let query = FakeClassQuery;
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_script_extensions_invalid() {
    struct FakeClassQuery;

    impl ClassQuery<'_> for FakeClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery<'_>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: "Script_Extensions",
                property_value: "InvalidScript",
            })
        }
    }

    let query = FakeClassQuery;
    let result = class(query);
    assert!(result.is_err());
}

