// Answer 0

#[derive(Debug)]
struct TestVisitor;

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }

    fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }

    fn visit_i32(self, _: i32) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }
}

#[test]
fn test_deserialize_ignored_any() {
    let visitor = TestVisitor;
    let result: Result<(), serde::de::Error> = deserialize_ignored_any(visitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_should_panic() {
    struct PanicVisitor;

    impl<'de> serde::de::Visitor<'de> for PanicVisitor {
        type Value = ();
        
        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            panic!("Panic triggered");
        }
    }

    let visitor = PanicVisitor;
    let _ = deserialize_ignored_any(visitor);
}

