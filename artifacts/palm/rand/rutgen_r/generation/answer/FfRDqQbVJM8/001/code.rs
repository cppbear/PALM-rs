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

    let instance = Mcg128Xsl64;
    let mut output = String::new();
    let result = fmt::write(&mut output, format_args!("{}", instance));
    
    assert!(result.is_ok());
    assert_eq!(output, "Mcg128Xsl64 {}\n");
}

