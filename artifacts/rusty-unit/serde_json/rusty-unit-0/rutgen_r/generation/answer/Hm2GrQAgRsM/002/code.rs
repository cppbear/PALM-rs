// Answer 0

#[derive(Default)]
struct TestDeserializer {
    should_fail: bool,
}

impl TestDeserializer {
    fn parse_object_colon(&mut self) -> Result<(), &'static str> {
        if self.should_fail {
            Err("parse_object_colon failed")
        } else {
            Ok(())
        }
    }
}

struct TestSeed;

impl<'de> serde::de::DeserializeSeed<'de> for TestSeed {
    type Value = i32;

    fn deserialize<T>(self, _: &mut T) -> Result<Self::Value, T::Error>
    where
        T: serde::de::Deserializer<'de>,
    {
        Ok(42)
    }
}

struct Context {
    de: TestDeserializer,
}

impl Context {
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut self.de)?;
        self.de.parse_object_colon()?;
        Ok((val, self))
    }
}

#[test]
fn test_variant_seed_should_return_err_on_parse_object_colon_failure() {
    let context = Context {
        de: TestDeserializer { should_fail: true },
    };
    let seed = TestSeed;

    let result = context.variant_seed(seed);

    assert!(result.is_err());
    assert_eq!(result.err(), Some("parse_object_colon failed"));
}

#[test]
fn test_variant_seed_should_succeed_when_no_failure() {
    let context = Context {
        de: TestDeserializer { should_fail: false },
    };
    let seed = TestSeed;

    let result = context.variant_seed(seed);

    assert!(result.is_ok());
    let (val, _) = result.unwrap();
    assert_eq!(val, 42);
}

