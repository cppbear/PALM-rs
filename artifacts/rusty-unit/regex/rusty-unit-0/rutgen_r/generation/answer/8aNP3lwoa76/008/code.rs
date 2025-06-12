// Answer 0

#[test]
fn test_class_binary() {
    struct TestClassQuery {
        name: &'static str,
    }

    impl ClassQuery<'static> for TestClassQuery {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'static>> {
            Ok(CanonicalClassQuery::Binary(self.name))
        }
    }

    let query = TestClassQuery { name: "SomeBinaryProperty" }; // replace with a valid binary property name
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_script() {
    struct TestClassQuery {
        name: &'static str,
    }

    impl ClassQuery<'static> for TestClassQuery {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'static>> {
            Ok(CanonicalClassQuery::Script(self.name))
        }
    }

    let query = TestClassQuery { name: "Latin" }; // replace with a valid script name
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_script_extensions() {
    struct TestClassQuery {
        property_name: &'static str,
        property_value: &'static str,
    }

    impl ClassQuery<'static> for TestClassQuery {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'static>> {
            Ok(CanonicalClassQuery::ByValue {
                property_name: self.property_name,
                property_value: self.property_value,
            })
        }
    }

    let query = TestClassQuery { property_name: "Script_Extensions", property_value: "Latin,Grek" }; // replace with valid values
    let result = class(query);
    assert!(result.is_ok());
}

