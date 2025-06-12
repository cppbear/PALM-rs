// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestNum {
        n: N,
    }

    enum N {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    impl std::fmt::Display for TestNum {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self.n {
                N::PosInt(u) => formatter.write_str(itoa::Buffer::new().format(u)),
                N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
                N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
            }
        }
    }

    #[test]
    fn test_neg_int() {
        let num = TestNum { n: N::NegInt(-42) };
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", num);
        assert!(result.is_ok());
        assert_eq!(buffer, "-42");
    }

    #[test]
    fn test_neg_int_zero() {
        let num = TestNum { n: N::NegInt(0) };
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", num);
        assert!(result.is_ok());
        assert_eq!(buffer, "0");
    }

    #[test]
    fn test_neg_int_smallest() {
        let num = TestNum { n: N::NegInt(i32::MIN) };
        let mut buffer = String::new();
        let result = write!(&mut buffer, "{}", num);
        assert!(result.is_ok());
        assert_eq!(buffer, "-2147483648");
    }
}

