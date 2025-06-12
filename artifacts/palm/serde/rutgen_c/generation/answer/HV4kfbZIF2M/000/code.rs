// Answer 0

#[test]
fn test_serialize_newtype_variant() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {}
    impl serde::ser::Error for DummyError {
        fn custom<T>(_: T) -> Self {
            DummyError
        }
    }

    let serializer = ContentSerializer::<DummyError> { error: std::marker::PhantomData };

    struct Newtype {
        value: i32,
    }

    impl serde::ser::Serialize for Newtype {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_i32(self.value)
        }
    }

    let newtype = Newtype { value: 42 };

    let result = serializer.serialize_newtype_variant::<Newtype>(
        "TestName",
        0,
        "TestVariant",
        &newtype,
    );
    
    match result {
        Ok(Content::NewtypeVariant(name, variant_index, variant, value)) => {
            assert_eq!(name, "TestName");
            assert_eq!(variant_index, 0);
            assert_eq!(variant, "TestVariant");
            if let Content::Some(value) = *value {
                assert_eq!(matches!(*value, Content::I32(42)), true);
            } else {
                panic!("Expected Newtype to be wrapped in Some");
            }
        }
        _ => panic!("Expected Ok with NewtypeVariant, got {:?}", result),
    }
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_with_non_serializable() {
    struct NonSerializable;

    let serializer = ContentSerializer::<DummyError> { error: std::marker::PhantomData };
    let result = serializer.serialize_newtype_variant::<NonSerializable>(
        "TestName",
        0,
        "TestVariant",
        &NonSerializable,
    );
    // This test expects a panic; intentionally not implemented for serialization.
    let _ = result.expect("Expected to panic due to non-serializable type");
}

