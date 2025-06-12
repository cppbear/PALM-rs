// Answer 0

#[test]
fn test_visit_str_valid() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid integer as a string")
        }
    }
    
    let result: Result<i32, _> = TestVisitor.visit_str("123");
    assert_eq!(result, Ok(123));
}

#[test]
fn test_visit_str_invalid() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid integer as a string")
        }
    }

    let result: Result<i32, _> = TestVisitor.visit_str("abc");
    assert!(result.is_err());
}

