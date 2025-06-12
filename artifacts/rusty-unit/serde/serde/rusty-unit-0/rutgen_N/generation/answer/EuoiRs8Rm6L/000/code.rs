// Answer 0

#[test]
fn test_expecting() {
    struct ExpectingFormatter {
        expecting: &'static str,
    }

    impl ExpectingFormatter {
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(self.expecting)
        }
    }
    
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        let instance = ExpectingFormatter { expecting: "test value" };
        instance.expecting(formatter).unwrap();
    }
    
    assert_eq!(output, "test value");
}

