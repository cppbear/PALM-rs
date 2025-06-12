// Answer 0

#[test]
fn test_missing_field_with_option() {
    use serde::de::{Deserialize, Deserializer, Visitor};
    use serde::de::{self, Error};
    use std::marker::PhantomData;

    struct TestOption<'de>(&'de str);

    impl<'de> Deserialize<'de> for TestOption<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let _: Option<()>;
            missing_field::<TestOption<'de>, D::Error>("test_field")?;
            Ok(TestOption("Not Used"))
        }
    }

    let result: Result<TestOption<'_>, _> = missing_field::<TestOption<'_>, de::value::Error>("test_field");
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "missing field `test_field`")]
fn test_missing_field_without_option() {
    use serde::de::{Deserialize, Deserializer, Visitor};
    use serde::de::{self, Error};
    use std::marker::PhantomData;

    struct TestNonOption<'de>(&'de str);

    impl<'de> Deserialize<'de> for TestNonOption<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            missing_field::<TestNonOption<'de>, D::Error>("test_field")?;
            Ok(TestNonOption("Not Used"))
        }
    }

    let result: Result<TestNonOption<'_>, _> = missing_field::<TestNonOption<'_>, de::value::Error>("test_field");
    assert!(result.is_err());
}

