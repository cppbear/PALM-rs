// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct Error;
    impl serde::Error for Error {
        fn custom<T>(_msg: T) -> Self {
            Error
        }
    }

    let serializer = ContentSerializer::<Error> {
        error: std::marker::PhantomData,
    };

    let name = "TestStruct";
    let result = serializer.serialize_unit_struct(name);
    assert_eq!(result, Ok(Content::UnitStruct(name)));
}

#[test]
fn test_serialize_unit_struct_empty_name() {
    struct Error;
    impl serde::Error for Error {
        fn custom<T>(_msg: T) -> Self {
            Error
        }
    }

    let serializer = ContentSerializer::<Error> {
        error: std::marker::PhantomData,
    };

    let name = "";
    let result = serializer.serialize_unit_struct(name);
    assert_eq!(result, Ok(Content::UnitStruct(name)));
}

