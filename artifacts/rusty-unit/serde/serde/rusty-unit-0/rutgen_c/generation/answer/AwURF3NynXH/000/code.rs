// Answer 0

#[test]
fn test_path_buf_visitor_expectation() {
    struct PathBufVisitor;

    impl<'de> Visitor<'de> for PathBufVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("path string")
        }
    }

    let visitor = PathBufVisitor;
    let mut result = String::new();
    let expectation = visitor.expecting(&mut result);

    assert!(expectation.is_ok());
    assert_eq!(result, "path string");
}

