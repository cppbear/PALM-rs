// Answer 0

#[test]
fn test_class_binary_query() {
    struct MyClassQuery<'a> {
        name: &'a str,
    }

    impl<'a> ClassQuery<'a> for MyClassQuery<'a> {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::Binary(self.name))
        }
    }

    let query = MyClassQuery { name: "my_binary_property" };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_query() {
    struct MyClassQuery<'a> {
        name: &'a str,
    }

    impl<'a> ClassQuery<'a> for MyClassQuery<'a> {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::GeneralCategory(self.name))
        }
    }

    let query = MyClassQuery { name: "Lu" }; // Uppercase letters
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_assigned() {
    struct MyClassQuery<'a> {
        name: &'a str,
    }

    impl<'a> ClassQuery<'a> for MyClassQuery<'a> {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::GeneralCategory("Assigned"))
        }
    }

    let query = MyClassQuery { name: "Assigned" };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_any_should_fail() {
    struct MyClassQuery<'a> {
        name: &'a str,
    }

    impl<'a> ClassQuery<'a> for MyClassQuery<'a> {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::GeneralCategory("Any"))
        }
    }

    let query = MyClassQuery { name: "Any" };
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_general_category_ascii_should_fail() {
    struct MyClassQuery<'a> {
        name: &'a str,
    }

    impl<'a> ClassQuery<'a> for MyClassQuery<'a> {
        fn canonicalize(&self) -> Result<CanonicalClassQuery<'a>> {
            Ok(CanonicalClassQuery::GeneralCategory("ASCII"))
        }
    }

    let query = MyClassQuery { name: "ASCII" };
    let result = class(query);
    assert!(result.is_err());
}

