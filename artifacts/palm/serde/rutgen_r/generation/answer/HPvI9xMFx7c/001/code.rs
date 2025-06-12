// Answer 0

#[test]
fn test_visit_enum_variant_err() {
    use serde::de::{EnumAccess, Visitor};
    use serde::de::IgnoredAny;
    use serde::de::Error as DeError;

    struct MockEnumAccess {
        should_err: bool,
    }

    impl<'de> EnumAccess<'de> for MockEnumAccess {
        type Error = TestError;
        type Variant = MockVariant;

        fn variant<A>(self) -> Result<(A, Self::Variant), Self::Error>
        where
            A: Visitor<'de>,
        {
            if self.should_err {
                Err(TestError::custom("Variant error"))
            } else {
                Ok((MockVariant, MockVariant))
            }
        }
    }

    struct MockVariant;

    struct TestError {
        message: String,
    }

    impl DeError for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError {
                message: msg.to_string(),
            }
        }
    }

    let mock_data = MockEnumAccess { should_err: true };
    let result: Result<_, _> = visit_enum(mock_data);
    assert!(result.is_err());
}

fn visit_enum<A>(data: A) -> Result<IgnoredAny, A::Error>
where
    A: EnumAccess<'de>,
{
    data.variant::<IgnoredAny>().map_err(|err| err)
}

