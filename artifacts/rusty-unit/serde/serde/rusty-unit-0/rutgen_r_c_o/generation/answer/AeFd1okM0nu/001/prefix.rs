// Answer 0

#[test]
fn test_serialize_value_null() {
    let mut serializer = Impossible::<T, Error> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let value: &Option<()> = &None;
    let _ = serializer.serialize_value(value);
}

#[test]
fn test_serialize_value_valid_instance() {
    #[derive(Serialize)]
    struct TestStruct {
        field: i32,
    }

    let mut serializer = Impossible::<T, Error> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let value = TestStruct { field: 42 };
    let _ = serializer.serialize_value(&value);
}

#[test]
#[should_panic]
fn test_serialize_value_panic() {
    let mut serializer = Impossible::<T, Error> {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };
    let value: &str = "Panic value";
    let _ = serializer.serialize_value(value);
}

