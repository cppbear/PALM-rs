// Answer 0

#[test]
fn test_deserialize_human_readable() {
    use serde::de::{Deserializer, Visitor};
    use std::str::FromStr;
    use std::net::SocketAddr;
    use serde::Deserializer as SerdeDeserializer;

    struct MockDeserializer;

    impl SerdeDeserializer for MockDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_str("127.0.0.1:8080")
        }

        fn is_human_readable(&self) -> bool {
            true
        }
    }

    let deserializer = MockDeserializer;
    let result: Result<SocketAddr, _> = deserialize(deserializer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "127.0.0.1:8080".parse::<SocketAddr>().unwrap());
}

#[test]
fn test_deserialize_non_human_readable() {
    use serde::de::{Deserializer, Visitor};
    use std::net::SocketAddr;
    use serde::Deserializer as SerdeDeserializer;

    struct MockDeserializer;

    impl SerdeDeserializer for MockDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_enum(Variant::new("V4")) // Simulating V4 as selected
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }
    
    struct Variant<'de> {
        value: &'de str,
    }

    impl<'de> Visitor<'de> for Variant<'de> {
        type Value = SocketAddr;

        fn visit_enum<E>(self, value: E) -> Result<Self::Value, Self::Error>
        where
            E: serde::de::EnumAccess<'de>,
        {
            match value.variant()? {
                "V4" => Ok("127.0.0.1:8080".parse::<SocketAddr>().unwrap()),
                "V6" => Ok("[::1]:8080".parse::<SocketAddr>().unwrap()),
                _ => Err(serde::de::value::Error::custom("unknown variant")),
            }
        }
    }

    let deserializer = MockDeserializer;
    let result: Result<SocketAddr, _> = deserialize(deserializer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "127.0.0.1:8080".parse::<SocketAddr>().unwrap());
}

