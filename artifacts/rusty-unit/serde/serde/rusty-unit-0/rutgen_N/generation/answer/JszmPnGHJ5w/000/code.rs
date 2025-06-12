// Answer 0

#[test]
fn test_visit_str_success() {
    struct MyVisitor;
    
    impl MyVisitor {
        fn visit_str<E>(self, v: &str) -> Result<(), E>
        where
            E: serde::de::Error,
        {
            CString::new(v).map_err(|_| E::custom("failed to create CString"))?;
            Ok(())
        }
    }

    let visitor = MyVisitor;
    let result = visitor.visit_str("valid string");
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "failed to create CString")]
fn test_visit_str_failure() {
    struct MyVisitor;

    impl MyVisitor {
        fn visit_str<E>(self, v: &str) -> Result<(), E>
        where
            E: serde::de::Error,
        {
            CString::new(v).map_err(|_| E::custom("failed to create CString"))?;
            Ok(())
        }
    }

    let visitor = MyVisitor;
    let _ = visitor.visit_str("\0invalid string");
}

