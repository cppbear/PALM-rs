// Answer 0

#[test]
fn test_cow_str_deserializer_owned() {
    use std::borrow::Cow;
    use serde::de::{self, Visitor};

    struct MockVisitor {
        visited_string: Option<String>
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            self.visited_string = Some(value.clone());
            Ok(value)
        }
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
    }

    let owned_string = String::from("hello, world");
    let deserializer = CowStrDeserializer {
        value: Cow::Owned(owned_string.clone()),
        marker: std::marker::PhantomData,
    };
    
    let visitor = MockVisitor { visited_string: None };
    let result = deserializer.deserialize_any(visitor).unwrap();
    
    assert_eq!(result, owned_string);
}

#[test]
fn test_cow_str_deserializer_borrowed() {
    use std::borrow::Cow;
    use serde::de::{self, Visitor};

    struct MockVisitor {
        visited_string: Option<String>
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            self.visited_string = Some(value.clone());
            Ok(value)
        }
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }
    }

    let borrowed_string = "hello, world";
    let deserializer = CowStrDeserializer {
        value: Cow::Borrowed(borrowed_string),
        marker: std::marker::PhantomData,
    };
    
    let visitor = MockVisitor { visited_string: None };
    let result = deserializer.deserialize_any(visitor).unwrap();
    
    assert_eq!(result, borrowed_string.to_owned());
}

