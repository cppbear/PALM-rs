// Answer 0

#[test]
fn test_serialize_key_success() {
    struct MockFormatter {
        called_begin: bool,
        called_end: bool,
    }

    impl MockFormatter {
        fn begin_object_key(&mut self, _writer: &mut Vec<u8>, _is_first: bool) -> Result<(), ()> {
        self.called_begin = true;
        Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
        self.called_end = true;
        Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    struct MockState {
        pub state: State,
    }

    let mut ser = MockSerializer {
        formatter: MockFormatter { called_begin: false, called_end: false },
        writer: Vec::new(),
    };
    
    let mut state = MockState { state: State::First };

    let result = serialize_key(&mut ser, &"test_key");
    
    assert!(result.is_ok());
    assert!(ser.formatter.called_begin);
    assert!(ser.formatter.called_end);
    assert_eq!(state.state, State::Rest);
}

#[test]
#[should_panic]
fn test_serialize_key_panic_on_error() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_object_key(&mut self, _writer: &mut Vec<u8>, _is_first: bool) -> Result<(), ()> {
            Err(())
        }

        fn end_object_key(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    let mut ser = MockSerializer {
        formatter: MockFormatter,
        writer: Vec::new(),
    };
    
    let mut state = State::First;

    let result = serialize_key(&mut ser, &"test_key");

    assert!(result.is_err());
}

#[test]
fn test_serialize_key_err_on_key_serialize() {
    struct MockKey;

    impl Serialize for MockKey {
        fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(S::Error::custom("Serialization Error"))
        }
    }

    struct MockFormatter {
        called_begin: bool,
        called_end: bool,
    }

    impl MockFormatter {
        fn begin_object_key(&mut self, _writer: &mut Vec<u8>, _is_first: bool) -> Result<(), ()> {
            self.called_begin = true;
            Ok(())
        }

        fn end_object_key(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            self.called_end = true;
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    let mut ser = MockSerializer {
        formatter: MockFormatter { called_begin: false, called_end: false },
        writer: Vec::new(),
    };
    
    let mut state = State::First;

    let result = serialize_key(&mut ser, &MockKey);
    
    assert!(result.is_err());
    assert!(ser.formatter.called_begin);
    assert!(!ser.formatter.called_end);
}

