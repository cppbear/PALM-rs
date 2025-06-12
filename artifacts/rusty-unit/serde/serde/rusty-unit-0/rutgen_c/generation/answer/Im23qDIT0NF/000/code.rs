// Answer 0

#[test]
fn test_serialize_tuple_variant_success() {
    struct MockSerializer {
        called: bool,
        entries: Vec<(Content, Content)>,
    }

    impl SerializeMap for MockSerializer {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = MockSerializer {
        called: false,
        entries: Vec::new(),
    };

    let tag = "tag";
    let variant_name = "variant_name";

    let tagged_serializer = TaggedSerializer {
        type_ident: "type_ident",
        variant_ident: "variant_ident",
        tag,
        variant_name,
        delegate: serializer,
    };

    let result = tagged_serializer.serialize_tuple_variant("some_str", 0, "inner_variant", 1);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_tuple_variant_failure() {
    struct FailingSerializer;

    impl SerializeMap for FailingSerializer {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(Error)
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = FailingSerializer;

    let tag = "tag";
    let variant_name = "variant_name";

    let tagged_serializer = TaggedSerializer {
        type_ident: "type_ident",
        variant_ident: "variant_ident",
        tag,
        variant_name,
        delegate: serializer,
    };

    tagged_serializer
        .serialize_tuple_variant("some_str", 0, "inner_variant", 1)
        .unwrap();
}

