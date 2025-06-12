// Answer 0

#[test]
fn test_serialize_newtype_variant_success() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn begin_object_key(&mut self, _writer: &mut (), _first: bool) -> Result<(), ()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn begin_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: (),
    }

    impl Serializer {
        fn serialize_str(&self, _: &'static str) -> Result<(), ()> { Ok(()) }
    }

    let serializer = Serializer {
        formatter: DummyFormatter,
        writer: (),
    };

    let result = serialize_newtype_variant(&serializer, "name", 0, "variant", &"value");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_fail_begin_object() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Err(()) }
        fn begin_object_key(&mut self, _writer: &mut (), _first: bool) -> Result<(), ()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn begin_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: (),
    }

    impl Serializer {
        fn serialize_str(&self, _: &'static str) -> Result<(), ()> { Ok(()) }
    }

    let serializer = Serializer {
        formatter: DummyFormatter,
        writer: (),
    };

    let _ = serialize_newtype_variant(&serializer, "name", 0, "variant", &"value");
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_fail_serialize_str() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn begin_object_key(&mut self, _writer: &mut (), _first: bool) -> Result<(), ()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn begin_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: (),
    }

    impl Serializer {
        fn serialize_str(&self, _: &'static str) -> Result<(), ()> { Err(()) }
    }

    let serializer = Serializer {
        formatter: DummyFormatter,
        writer: (),
    };

    let _ = serialize_newtype_variant(&serializer, "name", 0, "variant", &"value");
}

#[test]
#[should_panic]
fn test_serialize_newtype_variant_fail_end_object_key() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn begin_object_key(&mut self, _writer: &mut (), _first: bool) -> Result<(), ()> { Ok(()) }
        fn end_object_key(&mut self, _writer: &mut ()) -> Result<(), ()> { Err(()) }
        fn begin_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object_value(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
        fn end_object(&mut self, _writer: &mut ()) -> Result<(), ()> { Ok(()) }
    }

    struct Serializer {
        formatter: DummyFormatter,
        writer: (),
    }

    impl Serializer {
        fn serialize_str(&self, _: &'static str) -> Result<(), ()> { Ok(()) }
    }

    let serializer = Serializer {
        formatter: DummyFormatter,
        writer: (),
    };

    let _ = serialize_newtype_variant(&serializer, "name", 0, "variant", &"value");
}

