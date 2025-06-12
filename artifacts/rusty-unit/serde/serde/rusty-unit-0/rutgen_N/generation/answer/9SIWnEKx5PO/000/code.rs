// Answer 0

#[test]
fn test_expecting() {
    struct Dummy;

    impl std::fmt::Display for Dummy {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            // This function won't be tested, but it must exist for the trait.
            write!(formatter, "dummy")
        }
    }

    let dummy = Dummy;
    let mut buffer = String::new();
    let result = dummy.expecting(&mut std::fmt::Formatter::new(&mut buffer));
    
    assert!(result.is_ok());
    assert_eq!(buffer, "a borrowed path");
}

