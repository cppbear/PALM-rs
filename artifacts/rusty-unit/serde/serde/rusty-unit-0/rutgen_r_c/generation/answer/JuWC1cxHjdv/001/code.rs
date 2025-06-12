// Answer 0

#[test]
fn test_visit_f32() {
    struct TestError;

    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestVisitor<'de> {
        _phantom: PhantomData<&'de ()>,
    }

    impl<'de> Visitor<'de> for TestVisitor<'de> {
        type Value = TagOrContent<'de>;

        fn expecting(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(TagOrContent::Content(Content::F32(value)))
        }
    }

    let visitor = TestVisitor {
        _phantom: PhantomData,
    };

    // Test normal f32 values
    assert_eq!(
        visitor.visit_f32(0.0).unwrap(),
        TagOrContent::Content(Content::F32(0.0))
    );
    assert_eq!(
        visitor.visit_f32(1.0).unwrap(),
        TagOrContent::Content(Content::F32(1.0))
    );
    assert_eq!(
        visitor.visit_f32(-1.0).unwrap(),
        TagOrContent::Content(Content::F32(-1.0))
    );

    // Test maximum and minimum f32 values
    assert_eq!(
        visitor.visit_f32(f32::MAX).unwrap(),
        TagOrContent::Content(Content::F32(f32::MAX))
    );
    assert_eq!(
        visitor.visit_f32(f32::MIN).unwrap(),
        TagOrContent::Content(Content::F32(f32::MIN))
    );

    // Test NaN and Infinity
    let result_nan = visitor.visit_f32(f32::NAN);
    assert!(result_nan.is_ok());  // Ensure no panic occurs

    let result_infinity = visitor.visit_f32(f32::INFINITY);
    assert!(result_infinity.is_ok());  // Ensure no panic occurs

    let result_neg_infinity = visitor.visit_f32(f32::NEG_INFINITY);
    assert!(result_neg_infinity.is_ok());  // Ensure no panic occurs
}

