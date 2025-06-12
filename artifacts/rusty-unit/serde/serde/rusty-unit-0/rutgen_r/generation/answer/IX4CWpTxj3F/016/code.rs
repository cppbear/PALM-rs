// Answer 0

#[test]
fn test_signed_variant() {
    use std::fmt;

    struct Unexpected {
        value: i32,
    }

    impl fmt::Debug for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "integer `{}`", self.value)
        }
    }

    let signed_variants = vec![-1, 0, 1, i32::MIN, i32::MAX];
    
    for value in signed_variants {
        let variant = Unexpected { value };
        let mut output = String::new();
        let result = variant.fmt(&mut fmt::Formatter::new(&mut output));
        assert!(result.is_ok());
        assert_eq!(output, format!("integer `{}`", value));
    }
}

