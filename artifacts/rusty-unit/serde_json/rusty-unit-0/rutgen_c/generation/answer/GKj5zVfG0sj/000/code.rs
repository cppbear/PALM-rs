// Answer 0

#[test]
fn test_deserialize_struct_empty_object() {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let input = b"{ }";
    let deserializer = Deserializer {
        read: input.as_ref(),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result: Result<()> = deserializer.deserialize_struct("test", &[], Visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_struct_eof() {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: de::MapAccess<'de>,
        {
            Err(Error::syntax(ErrorCode::EofWhileParsingObject))
        }
    }

    let input = b"{";
    let deserializer = Deserializer {
        read: input.as_ref(),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result: Result<()> = deserializer.deserialize_struct("test", &[], Visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_struct_invalid_type() {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = ();

        fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    let input = b"[";
    let deserializer = Deserializer {
        read: input.as_ref(),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result: Result<()> = deserializer.deserialize_struct("test", &[], Visitor);
    assert!(result.is_err());
}

