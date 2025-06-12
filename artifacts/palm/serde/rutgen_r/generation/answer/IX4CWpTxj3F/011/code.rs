// Answer 0

#[test]
fn test_fmt_unit() {
    use std::fmt;

    struct Unexpected;

    impl fmt::Debug for Unexpected {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("unit value")
        }
    }

    let unit_value = Unexpected; 
    let mut output = String::new();
    let result = fmt::Write::write_str(&mut output, &unit_value);
    
    assert!(result.is_ok());
    assert_eq!(output, "unit value");
}

