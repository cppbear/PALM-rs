// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use std::string::ToString;

    struct Lcg64Xsh32;

    impl fmt::Display for Lcg64Xsh32 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg64Xsh32 {{}}")
        }
    }

    let lcg = Lcg64Xsh32;
    let result = format!("{}", lcg);
    assert_eq!(result, "Lcg64Xsh32 {}");
}

#[test]
fn test_fmt_empty() {
    use std::fmt;

    struct Lcg64Xsh32;

    impl fmt::Display for Lcg64Xsh32 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg64Xsh32 {{}}")
        }
    }

    let lcg = Lcg64Xsh32;
    let result = format!("{}", lcg);
    assert_eq!(result, "Lcg64Xsh32 {}");
}

