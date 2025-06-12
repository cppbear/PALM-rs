// Answer 0

#[test]
fn test_phantom_data_visitor_expecting() {
    use std::fmt::Formatter;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = PhantomData<()>;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter.write_str("unit")
        }
    }
    
    let mut formatter = Formatter::new();
    let visitor = TestVisitor;

    assert!(visitor.expecting(&mut formatter).is_ok());
    assert_eq!(formatter.to_string(), "unit");
}

