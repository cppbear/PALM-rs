// Answer 0

#[test]
fn test_fmt_empty_array() {
    struct Array64;

    impl std::fmt::Display for Array64 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Array64 {{}}")
        }
    }

    let array = Array64;
    let result = format!("{}", array);
    assert_eq!(result, "Array64 {}");
}

#[test]
fn test_fmt_multiple_times() {
    struct Array64;

    impl std::fmt::Display for Array64 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Array64 {{}}")
        }
    }

    let array = Array64;
    let result1 = format!("{}", array);
    let result2 = format!("{}", array);
    assert_eq!(result1, "Array64 {}");
    assert_eq!(result2, "Array64 {}");
}

#[test]
#[should_panic]
fn test_fmt_panic_condition() {
    struct Array64;

    impl std::fmt::Display for Array64 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            panic!("Intentional panic for testing")
        }
    }

    let array = Array64;
    let _ = format!("{}", array); // This should panic
}

