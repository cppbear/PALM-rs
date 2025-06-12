// Answer 0

#[test]
fn test_next_element_seed_with_first_element() {
    struct SeedWithBool;
    impl<'de> de::DeserializeSeed<'de> for SeedWithBool {
        type Value = bool;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_any(BoolDeserializer::new(true))
        }
    }

    let mut visitor = PairVisitor(Some(1), None, PhantomData);
    let result = visitor.next_element_seed(SeedWithBool);
}

#[test]
fn test_next_element_seed_with_second_element() {
    struct SeedWithI32;
    impl<'de> de::DeserializeSeed<'de> for SeedWithI32 {
        type Value = i32;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_any(I32Deserializer::new(2))
        }
    }

    let mut visitor = PairVisitor(None, Some(3), PhantomData);
    let result = visitor.next_element_seed(SeedWithI32);
}

#[test]
fn test_next_element_seed_with_both_elements() {
    struct SeedWithString;
    impl<'de> de::DeserializeSeed<'de> for SeedWithString {
        type Value = String;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_any(StrDeserializer::new("test"))
        }
    }

    let mut visitor = PairVisitor(Some(1), Some("test"), PhantomData);
    let result1 = visitor.next_element_seed(SeedWithString);
    let result2 = visitor.next_element_seed(SeedWithBool);
}

#[should_panic]
#[test]
fn test_next_element_seed_with_no_elements() {
    struct SeedWithChar;
    impl<'de> de::DeserializeSeed<'de> for SeedWithChar {
        type Value = char;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_any(CharDeserializer::new('a'))
        }
    }

    let mut visitor = PairVisitor(None, None, PhantomData);
    let result = visitor.next_element_seed(SeedWithChar);
}

