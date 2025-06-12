// Answer 0

#[test]
fn test_serialize_element_ok_first() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array_value(&mut self, _writer: &mut Vec<u8>, _first: bool) -> Result<(), Error> {
            Ok(())
        }
        
        fn end_array_value(&mut self, _writer: &mut Vec<u8>) -> Result<(), Error> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl<'a> ser::Serialize for MockSerializer {
        fn serialize<S>(&self, _serializer: &mut S) -> Result<(), Error>
        where
            S: ser::Serializer,
        {
            Ok(())
        }
    }

    let mut serializer = MockSerializer {
        writer: vec![],
        formatter: MockFormatter,
    };

    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: State::First,
    };

    let result = compound.serialize_element(&42);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_element_err() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array_value(&mut self, _writer: &mut Vec<u8>, _first: bool) -> Result<(), Error> {
            Ok(())
        }

        fn end_array_value(&mut self, _writer: &mut Vec<u8>) -> Result<(), Error> {
            Ok(())
        }
    }

    struct ErroneousValue;

    impl ser::Serialize for ErroneousValue {
        fn serialize<S>(&self, _serializer: &mut S) -> Result<(), Error>
        where
            S: ser::Serializer,
        {
            Err(Error)
        }
    }

    struct MockSerializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl<'a> ser::Serialize for MockSerializer {
        fn serialize<S>(&self, _serializer: &mut S) -> Result<(), Error>
        where
            S: ser::Serializer,
        {
            Ok(())
        }
    }

    let mut serializer = MockSerializer {
        writer: vec![],
        formatter: MockFormatter,
    };

    let mut compound = Compound::Map {
        ser: &mut serializer,
        state: State::First,
    };

    let _ = compound.serialize_element(&ErroneousValue);
}

