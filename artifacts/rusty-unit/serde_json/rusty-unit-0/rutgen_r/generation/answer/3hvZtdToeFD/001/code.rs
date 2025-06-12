// Answer 0

#[test]
fn test_next_value_seed_parse_object_colon_err() {
    struct MockDeserializer {
        has_error: bool,
    }

    impl serde::de::Deserializer<'_> for MockDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'_>,
        {
            Err(serde::de::value::Error::custom("deserialization error"))
        }

        fn parse_object_colon(&mut self) -> Result<(), Self::Error> {
            if self.has_error {
                Err(serde::de::value::Error::custom("parse error"))
            } else {
                Ok(())
            }
        }
    }

    struct TestSeed;

    impl<'de> serde::de::DeserializeSeed<'de> for TestSeed {
        type Value = usize;

        fn deserialize<DE>(
            self,
            de: &mut DE,
        ) -> Result<Self::Value, DE::Error>
        where
            DE: serde::de::Deserializer<'de>,
        {
            Ok(42)
        }
    }

    let mut de = MockDeserializer { has_error: true };
    let seed = TestSeed;

    let result: Result<usize, serde::de::value::Error> = de.next_value_seed(seed);
    assert!(result.is_err());
}

