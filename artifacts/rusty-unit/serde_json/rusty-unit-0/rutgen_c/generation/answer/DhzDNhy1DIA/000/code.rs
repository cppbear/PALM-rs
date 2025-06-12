// Answer 0

#[test]
fn test_expecting() {
    struct TestVisitor;

    impl fmt::Display for TestVisitor {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestVisitor")
        }
    }

    let classifier = KeyClassifier;
    let mut formatter = String::new();
    let result = classifier.expecting(&mut formatter);

    assert!(result.is_ok());
    assert_eq!(formatter, "a string key");
}

