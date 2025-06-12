// Answer 0

fn test_serialize_element_success() {
    use serde::ser::{Serializer, Serialize};
    use serde_json::ser::{Serializer as JsonSerializer, State, Compound};

    struct TestValue;

    impl Serialize for TestValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str("test_value")
        }
    }

    struct TestWriter;

    impl serde_json::ser::Writer for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
    }

    let mut writer = TestWriter;
    let formatter = JsonSerializer::new(&mut writer);
    let mut state = State::First;
    let mut ser = Compound::Map { ser: formatter, state: &mut state };

    let result = ser.serialize_element(&TestValue);
    assert!(result.is_ok());
}

fn test_serialize_element_failure() {
    use serde::ser::{Serializer, Serialize};
    use serde_json::ser::{Serializer as JsonSerializer, State, Compound};

    struct FailingValue;

    impl Serialize for FailingValue {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom("serialization error"))
        }
    }

    struct TestWriter;

    impl serde_json::ser::Writer for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
    }

    let mut writer = TestWriter;
    let formatter = JsonSerializer::new(&mut writer);
    let mut state = State::First;
    let mut ser = Compound::Map { ser: formatter, state: &mut state };

    let result = ser.serialize_element(&FailingValue);
    assert!(result.is_err());
}

