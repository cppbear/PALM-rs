// Answer 0

#[test]
fn test_canonical_binary_property_found() {
    struct ClassQueryTest<'a> {
        query: ClassQuery<'a>,
    }

    let class_query = ClassQueryTest {
        query: ClassQuery::Binary("Latin"),
    };

    let result = class_query.query.canonical_binary("Latin");
    assert!(result.is_ok());
}

#[test]
fn test_canonical_binary_general_category_found() {
    struct ClassQueryTest<'a> {
        query: ClassQuery<'a>,
    }

    let class_query = ClassQueryTest {
        query: ClassQuery::Binary("assigned"),
    };

    let result = class_query.query.canonical_binary("assigned");
    assert!(result.is_ok());
}

#[test]
fn test_canonical_binary_script_found() {
    struct ClassQueryTest<'a> {
        query: ClassQuery<'a>,
    }

    let class_query = ClassQueryTest {
        query: ClassQuery::Binary("Latin"),
    };

    let result = class_query.query.canonical_binary("Latin");
    assert!(result.is_ok());
    
    if let Ok(CanonicalClassQuery::Script(canon)) = result {
        assert_eq!(canon, "Latin");
    }
}

#[test]
fn test_canonical_binary_property_not_found() {
    struct ClassQueryTest<'a> {
        query: ClassQuery<'a>,
    }

    let class_query = ClassQueryTest {
        query: ClassQuery::Binary("UnknownProperty"),
    };

    let result = class_query.query.canonical_binary("UnknownProperty");
    assert!(result.is_err());
}

