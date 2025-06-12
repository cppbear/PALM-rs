// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct Dummy;

    impl fmt::Debug for Dummy {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Dummy")
        }
    }

    let mut formatter = fmt::Formatter::new();
    let dummy = Dummy;

    let result = dummy.expecting(&mut formatter);
    assert!(result.is_ok());

    let output = format!("{}", formatter);
    assert_eq!(output, "unit");
}

