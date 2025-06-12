// Answer 0

#[test]
fn test_serialize_newtype_struct_err() {
    struct ErroneousType;

    impl Serialize for ErroneousType {
        fn serialize<S>(&self, _: S) -> Result<Self::Ok, Self::Error>
        where
            S: Serializer,
        {
            Err(Error::custom("serialization error"))
        }
    }

    let serializer = ContentSerializer::<std::io::Error> { error: PhantomData };
    let result = serializer.serialize_newtype_struct("ErroneousType", &ErroneousType);
}

#[test]
fn test_serialize_newtype_struct_empty() {
    struct Empty;

    impl Serialize for Empty {
        fn serialize<S>(&self, _: S) -> Result<Self::Ok, Self::Error>
        where
            S: Serializer,
        {
            Err(Error::custom("serialization error"))
        }
    }

    let serializer = ContentSerializer::<std::io::Error> { error: PhantomData };
    let result = serializer.serialize_newtype_struct("Empty", &Empty);
}

#[test]
fn test_serialize_newtype_struct_nested() {
    struct NestedStruct;

    impl Serialize for NestedStruct {
        fn serialize<S>(&self, _: S) -> Result<Self::Ok, Self::Error>
        where
            S: Serializer,
        {
            Err(Error::custom("serialization error"))
        }
    }

    let serializer = ContentSerializer::<std::io::Error> { error: PhantomData };
    let result = serializer.serialize_newtype_struct("NestedStruct", &NestedStruct);
}

#[test]
fn test_serialize_newtype_struct_array() {
    struct ArrayStruct;

    impl Serialize for ArrayStruct {
        fn serialize<S>(&self, _: S) -> Result<Self::Ok, Self::Error>
        where
            S: Serializer,
        {
            Err(Error::custom("serialization error"))
        }
    }

    let serializer = ContentSerializer::<std::io::Error> { error: PhantomData };
    let result = serializer.serialize_newtype_struct("ArrayStruct", &ArrayStruct);
}

