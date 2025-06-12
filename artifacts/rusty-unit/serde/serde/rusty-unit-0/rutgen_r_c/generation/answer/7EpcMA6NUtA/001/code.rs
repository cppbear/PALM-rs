// Answer 0

#[test]
fn test_next_element_seed_some() {
    use serde::de::{self, DeserializeSeed, Visitor, IntoDeserializer};
    use std::iter::once;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: de::Deserializer<'de>,
        {
            deserializer.deserialize_i32(TestVisitor)
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer")
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    let deserializer = SeqDeserializer {
        iter: once(42).fuse(),
        count: 0,
        marker: PhantomData::<()>,
    };

    let mut deserializer = deserializer;
    let result: Result<Option<i32>, Box<str>> = deserializer.next_element_seed(TestSeed);

    assert_eq!(result, Ok(Some(42)));
    assert_eq!(deserializer.count, 1);
}

#[test]
fn test_next_element_seed_none() {
    use serde::de::{self, DeserializeSeed, Visitor, IntoDeserializer};

    struct EmptySeed;

    impl<'de> DeserializeSeed<'de> for EmptySeed {
        type Value = ();

        fn deserialize<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
        where
            T: de::Deserializer<'de>,
        {
            deserializer.deserialize_unit()
        }
    }

    let deserializer = SeqDeserializer {
        iter: std::iter::empty::<i32>().fuse(),
        count: 0,
        marker: PhantomData::<()>,
    };

    let mut deserializer = deserializer;
    let result: Result<Option<()>, Box<str>> = deserializer.next_element_seed(EmptySeed);

    assert_eq!(result, Ok(None));
    assert_eq!(deserializer.count, 0);
}

