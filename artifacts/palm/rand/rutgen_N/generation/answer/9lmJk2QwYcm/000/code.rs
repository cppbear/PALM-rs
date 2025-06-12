// Answer 0

#[cfg(test)]
mod tests {
    use std::fmt;

    struct Lcg64Xsh32;

    impl fmt::Display for Lcg64Xsh32 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg64Xsh32 {{}}")
        }
    }

    #[test]
    fn test_fmt() {
        let lcg = Lcg64Xsh32;
        let result = format!("{}", lcg);
        assert_eq!(result, "Lcg64Xsh32 {{}}");
    }
    
    #[should_panic]
    fn test_fmt_invalid() {
        let lcg = Lcg64Xsh32;
        let _ = format!("{:invalid}", lcg);
    }
}

