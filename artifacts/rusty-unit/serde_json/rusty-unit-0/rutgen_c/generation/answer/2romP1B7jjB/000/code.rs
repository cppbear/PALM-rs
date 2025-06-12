// Answer 0

#[test]
fn test_from_str_pos_int() {
    struct DummyVisitor;
    
    impl de::Visitor for DummyVisitor {
        type Value = Number;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number")
        }
    }
    
    let number: Result<Number, Error> = Number::from_str("42");
    assert_eq!(number.is_ok(), true);
}

#[test]
fn test_from_str_neg_int() {
    struct DummyVisitor;
    
    impl de::Visitor for DummyVisitor {
        type Value = Number;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number")
        }
    }
    
    let number: Result<Number, Error> = Number::from_str("-42");
    assert_eq!(number.is_ok(), true);
}

#[test]
fn test_from_str_float() {
    struct DummyVisitor;
    
    impl de::Visitor for DummyVisitor {
        type Value = Number;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number")
        }
    }
    
    let number: Result<Number, Error> = Number::from_str("3.14");
    assert_eq!(number.is_ok(), true);
}

#[test]
#[should_panic]
fn test_from_str_invalid() {
    let _number: Result<Number, Error> = Number::from_str("invalid");
}

