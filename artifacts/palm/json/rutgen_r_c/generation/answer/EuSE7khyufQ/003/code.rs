// Answer 0

#[test]
fn test_unexpected_pos_int() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Unexpected;

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Unexpected::Unsigned(v))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Unexpected::Signed(v))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Unexpected::Float(v))
        }

        fn visit_map<A>(self, _: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
            unimplemented!()
        }
    }

    let number = Number { n: N::PosInt(42) };
    let result = number.unexpected();  
    assert_eq!(result, Unexpected::Unsigned(42));
}

#[test]
fn test_unexpected_neg_int() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Unexpected;

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Unexpected::Unsigned(v))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Unexpected::Signed(v))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Unexpected::Float(v))
        }

        fn visit_map<A>(self, _: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
            unimplemented!()
        }
    }

    let number = Number { n: N::NegInt(-42) };
    let result = number.unexpected();  
    assert_eq!(result, Unexpected::Signed(-42));
}

#[test]
fn test_unexpected_float() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Unexpected;

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Unexpected::Unsigned(v))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Unexpected::Signed(v))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Unexpected::Float(v))
        }

        fn visit_map<A>(self, _: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
            unimplemented!()
        }
    }

    let number = Number { n: N::Float(3.14) };
    let result = number.unexpected();  
    assert_eq!(result, Unexpected::Float(3.14));
}

