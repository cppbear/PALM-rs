// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Mcg128Xsl64;

    impl fmt::Display for Mcg128Xsl64 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Mcg128Xsl64 {{}}")
        }
    }

    let mcg = Mcg128Xsl64;
    let result = format!("{}", mcg);
    assert_eq!(result, "Mcg128Xsl64 {}");
}

