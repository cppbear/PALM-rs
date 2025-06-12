// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Array64;

    impl fmt::Display for Array64 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Array64 {{}}")
        }
    }

    let array = Array64;
    let result = format!("{}", array);
    assert_eq!(result, "Array64 {}");
}

