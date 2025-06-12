// Answer 0

#[test]
fn test_serialize_newtype_variant_success() {
    struct MockSerializer {
        serialized: Vec<(String, String)>
    }

    impl SerializeMap for MockSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            self.serialized.push((key.serialize().unwrap(), String::new()));
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            if let Some(last_key) = self.serialized.last_mut() {
                last_key.1 = value.serialize().unwrap();
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    impl Serialize for String {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            serializer.serialize_str(self)
        }
    }

    let mut serialized_data = vec![];
    let mock_delegate = MockSerializer { serialized: serialized_data };
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variantName",
        delegate: mock_delegate,
    };

    let result: Result<(), ()> = serializer.serialize_newtype_variant("TestType", 0, "innerVariant", &"innerValue".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_serialize_newtype_variant_fail() {
    struct AlwaysFailingSerializer;

    impl SerializeMap for AlwaysFailingSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            Err(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
            Err(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
    }

    let failing_delegate = AlwaysFailingSerializer;

    let serializer = TaggedSerializer {
        type_ident: "FailTest",
        variant_ident: "FailVariant",
        tag: "tag",
        variant_name: "failedVariant",
        delegate: failing_delegate,
    };

    let result: Result<(), ()> = serializer.serialize_newtype_variant("FailTest", 0, "innerVariant", &"innerValue");
    assert!(result.is_err());
}

