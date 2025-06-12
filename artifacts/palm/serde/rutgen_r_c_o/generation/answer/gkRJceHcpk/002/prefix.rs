// Answer 0

#[test]
fn test_serialize_field_bool() {
    struct TestMap {
        ok: (),
        error: Option<Error>,
    }

    impl ser::SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = TestMap { ok: (), error: None };
    let mut serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![],
    };
    let value = Content::Bool(true);
    let key = "bool_field";
    let _ = serializer.serialize_field(key, &value);
}

#[test]
fn test_serialize_field_u8() {
    struct TestMap {
        ok: (),
        error: Option<Error>,
    }

    impl ser::SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = TestMap { ok: (), error: None };
    let mut serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![],
    };
    let value = Content::U8(255);
    let key = "u8_field";
    let _ = serializer.serialize_field(key, &value);
}

#[test]
fn test_serialize_field_string() {
    struct TestMap {
        ok: (),
        error: Option<Error>,
    }

    impl ser::SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = TestMap { ok: (), error: None };
    let mut serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![],
    };
    let value = Content::String(String::from("test"));
    let key = "string_field";
    let _ = serializer.serialize_field(key, &value);
}

#[test]
fn test_serialize_field_u32() {
    struct TestMap {
        ok: (),
        error: Option<Error>,
    }

    impl ser::SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = TestMap { ok: (), error: None };
    let mut serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![],
    };
    let value = Content::U32(12345);
    let key = "u32_field";
    let _ = serializer.serialize_field(key, &value);
}

#[test]
fn test_serialize_field_f64() {
    struct TestMap {
        ok: (),
        error: Option<Error>,
    }

    impl ser::SerializeMap for TestMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = TestMap { ok: (), error: None };
    let mut serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![],
    };
    let value = Content::F64(3.14);
    let key = "f64_field";
    let _ = serializer.serialize_field(key, &value);
}

