// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Lcg128Xsl64;

    impl fmt::Display for Lcg128Xsl64 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg128Xsl64 {{}}")
        }
    }

    let instance = Lcg128Xsl64;
    let output = format!("{}", instance);
    assert_eq!(output, "Lcg128Xsl64 {}");
}

