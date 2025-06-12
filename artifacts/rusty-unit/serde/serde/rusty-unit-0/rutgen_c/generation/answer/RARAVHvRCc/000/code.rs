// Answer 0

#[test]
fn test_array_visitor_expecting() {
    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("dummy visitor")
        }
    }

    let mut formatter = std::fmt::Formatter::new();
    let dummy = DummyVisitor;

    let result = dummy.expecting(&mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.into_string(), "dummy visitor");
}

#[test]
fn test_array_visitor_empty_array() {
    let marker = std::marker::PhantomData::<[(); 0]>;
    let array_visitor = ArrayVisitor { marker };

    let mut formatter = std::fmt::Formatter::new();
    let result = array_visitor.expecting(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.into_string(), "an empty array");
}

