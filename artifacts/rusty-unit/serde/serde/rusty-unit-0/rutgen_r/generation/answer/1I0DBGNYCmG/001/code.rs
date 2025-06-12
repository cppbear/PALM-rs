// Answer 0

#[test]
fn test_deserialize_float_invalid_type() {
    struct InvalidVisitor;
    
    impl<'de> serde::de::Visitor<'de> for InvalidVisitor {
        type Value = ();
        
        fn visit_f32<E>(self, _: f32) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected f32"))
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected f64"))
        }
        
        fn visit_u8<E>(self, _: u8) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected u8"))
        }
        
        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected u16"))
        }
        
        fn visit_u32<E>(self, _: u32) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected u32"))
        }
        
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected u64"))
        }
        
        fn visit_i8<E>(self, _: i8) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected i8"))
        }
        
        fn visit_i16<E>(self, _: i16) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected i16"))
        }
        
        fn visit_i32<E>(self, _: i32) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected i32"))
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> where E: serde::de::Error {
            Err(E::custom("Unexpected i64"))
        }
    }
    
    enum Content {
        Invalid
    }

    struct Deserializer {
        content: Box<Content>
    }
    
    impl Deserializer {
        fn invalid_type<V>(&self, _: &V) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match *self.content {
                Content::Invalid => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let deserializer = Deserializer {
        content: Box::new(Content::Invalid)
    };
    
    let result: Result<(), _> = deserializer.deserialize_float(InvalidVisitor {});
    
    assert!(result.is_err());
}

