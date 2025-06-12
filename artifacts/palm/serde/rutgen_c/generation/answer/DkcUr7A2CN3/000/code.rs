// Answer 0

#[test]
fn test_visit_f64() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
        fn visit_f64<E>(self, x: f64) -> Result<Self::Value, E> {
            let _ = x;
            Ok(IgnoredAny)
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_f64(3.14);
    assert_eq!(result.unwrap(), IgnoredAny);
}

#[test]
fn test_visit_f64_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
        fn visit_f64<E>(self, x: f64) -> Result<Self::Value, E> {
            let _ = x;
            Ok(IgnoredAny)
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_f64(0.0);
    assert_eq!(result.unwrap(), IgnoredAny);
}

#[test]
fn test_visit_f64_negative() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = IgnoredAny;
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
        fn visit_f64<E>(self, x: f64) -> Result<Self::Value, E> {
            let _ = x;
            Ok(IgnoredAny)
        }
    }

    let visitor = TestVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_f64(-2.718);
    assert_eq!(result.unwrap(), IgnoredAny);
}

