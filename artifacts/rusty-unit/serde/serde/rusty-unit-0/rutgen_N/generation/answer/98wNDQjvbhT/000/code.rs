// Answer 0

#[test]
fn test_visit_string_success() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_string<E>(self, v: String) -> Result<String, E>
        where
            E: std::fmt::Debug,
        {
            Ok(v)
        }
    }

    let visitor = TestVisitor;
    let input = String::from("test string");
    let result: Result<String, _> = visitor.visit_string(input);
    assert_eq!(result.unwrap(), "test string");
}

#[test]
#[should_panic]
fn test_visit_string_invalid() {
    struct TestVisitor;

    impl TestVisitor {
        fn visit_string<E>(self, _: String) -> Result<String, E>
        where
            E: std::fmt::Debug,
        {
            panic!();
        }
    }

    let visitor = TestVisitor;
    let _result: Result<String, _> = visitor.visit_string(String::from("test string"));
}

