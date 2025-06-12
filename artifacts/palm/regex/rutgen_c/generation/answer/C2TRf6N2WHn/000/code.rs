// Answer 0

#[test]
fn test_canonical_binary_valid_property() {
    struct TestClassQuery<'a>(&'a str);

    impl<'a> ClassQuery<'a> {
        fn new(binary: &'a str) -> Self {
            ClassQuery::Binary(binary)
        }
        
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery> {
            let norm = normalize(name);

            if let Some(canon) = canonical_prop(&norm) {
                return Ok(CanonicalClassQuery::Binary(canon));
            }
            if let Some(canon) = canonical_gencat(&norm) {
                return Ok(CanonicalClassQuery::GeneralCategory(canon));
            }
            if let Some(canon) = canonical_script(&norm) {
                return Ok(CanonicalClassQuery::Script(canon));
            }
            Err(Error::PropertyNotFound)
        }
    }

    let query = TestClassQuery::new("assigned");
    let result = query.canonical_binary("assigned").unwrap();
    assert_eq!(result, CanonicalClassQuery::GeneralCategory("Assigned"));
}

#[test]
fn test_canonical_binary_valid_script() {
    struct TestClassQuery<'a>(&'a str);

    impl<'a> ClassQuery<'a> {
        fn new(binary: &'a str) -> Self {
            ClassQuery::Binary(binary)
        }
        
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery> {
            let norm = normalize(name);

            if let Some(canon) = canonical_prop(&norm) {
                return Ok(CanonicalClassQuery::Binary(canon));
            }
            if let Some(canon) = canonical_gencat(&norm) {
                return Ok(CanonicalClassQuery::GeneralCategory(canon));
            }
            if let Some(canon) = canonical_script(&norm) {
                return Ok(CanonicalClassQuery::Script(canon));
            }
            Err(Error::PropertyNotFound)
        }
    }

    let query = TestClassQuery::new("script");
    let result = query.canonical_binary("greek").unwrap();
    assert_eq!(result, CanonicalClassQuery::Script("Greek"));
}

#[test]
fn test_canonical_binary_property_not_found() {
    struct TestClassQuery<'a>(&'a str);

    impl<'a> ClassQuery<'a> {
        fn new(binary: &'a str) -> Self {
            ClassQuery::Binary(binary)
        }
        
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery> {
            let norm = normalize(name);

            if let Some(canon) = canonical_prop(&norm) {
                return Ok(CanonicalClassQuery::Binary(canon));
            }
            if let Some(canon) = canonical_gencat(&norm) {
                return Ok(CanonicalClassQuery::GeneralCategory(canon));
            }
            if let Some(canon) = canonical_script(&norm) {
                return Ok(CanonicalClassQuery::Script(canon));
            }
            Err(Error::PropertyNotFound)
        }
    }

    let query = TestClassQuery::new("unknown");
    let result = query.canonical_binary("unknown_property");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::PropertyNotFound);
}

