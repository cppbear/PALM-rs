// Answer 0

#[test]
fn test_end_function() {
    struct Impossible;

    impl Impossible {
        fn end(self) -> Result<(), String> {
            match self {
                _ => Err("Cannot end".to_string()), // since we expect this to be impossible
            }
        }
    }

    let impossible = Impossible;
    let result = impossible.end();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cannot end");
}

