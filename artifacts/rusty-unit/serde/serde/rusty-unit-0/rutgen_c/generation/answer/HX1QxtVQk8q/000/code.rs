// Answer 0

#[test]
fn test_serialize_field_empty() {
    struct TestSerializeTupleVariant;

    impl ser::Error for Error {}

    impl SerializeTupleVariant for TestSerializeTupleVariant {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let _ = value;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut serializer = Impossible {
        void: {},
        ok: PhantomData,
        error: PhantomData,
    };

    let result: Result<(), Error> = serializer.serialize_field(&());
    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_serialize_field_panic() {
    struct TestSerializeTupleVariant;

    impl ser::Error for Error {}

    impl SerializeTupleVariant for TestSerializeTupleVariant {
        type Ok = ();
        type Error = Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let _ = value;
            panic!("intentional panic");
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut serializer = Impossible {
        void: {},
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = serializer.serialize_field(&());
}

