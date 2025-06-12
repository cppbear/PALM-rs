// Answer 0

#[test]
fn test_end_success_case_1() {
    struct MockMap {
        result: Result<(), Error>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.result.clone()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { result: Ok(()) };
    let fields = vec![
        ("field1", Content::U32(1)),
        ("field2", Content::String("value".to_string())),
    ];
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields,
    };

    let _ = serializer.end();
}

#[test]
fn test_end_success_case_2() {
    struct MockMap {
        result: Result<(), Error>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            self.result.clone()
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { result: Ok(()) };
    let fields = vec![
        ("fieldA", Content::I32(10)),
        ("fieldB", Content::Char('c')),
    ];
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "example",
        fields,
    };

    let _ = serializer.end();
}

#[test]
#[should_panic]
fn test_end_panic_case() {
    struct MockMap {
        result: Result<(), Error>,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { result: Err(Error) };
    let fields = vec![
        ("fieldX", Content::F64(3.14)),
        ("fieldY", Content::Bool(true)),
    ];
    let serializer = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields,
    };

    let _ = serializer.end();
}

