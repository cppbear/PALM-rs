// Answer 0

#[cfg(test)]
mod tests {
    use std::fmt;

    struct Number(i32);

    impl fmt::Display for Number {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Number({})", self.0)
        }
    }

    #[test]
    fn test_fmt() {
        let number = Number(42);
        let mut output = String::new();
        let result = write!(&mut output, "{}", number);
        assert!(result.is_ok());
        assert_eq!(output, "Number(42)");
    }

    #[test]
    fn test_fmt_zero() {
        let number = Number(0);
        let mut output = String::new();
        let result = write!(&mut output, "{}", number);
        assert!(result.is_ok());
        assert_eq!(output, "Number(0)");
    }

    #[test]
    fn test_fmt_negative() {
        let number = Number(-1);
        let mut output = String::new();
        let result = write!(&mut output, "{}", number);
        assert!(result.is_ok());
        assert_eq!(output, "Number(-1)");
    }
}

