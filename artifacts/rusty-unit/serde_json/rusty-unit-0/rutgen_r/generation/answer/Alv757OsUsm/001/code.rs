// Answer 0

#[derive(Default)]
struct MockSerializeSeq {
    finished: bool,
}

impl serde::ser::SerializeSeq for MockSerializeSeq {
    type Ok = super::Value;
    type Error = serde::ser::Error;

    fn serialize_element<T>(&mut self, _: T) -> Result<&mut Self>
    where
        T: serde::ser::Serialize,
    {
        Ok(self)
    }

    fn end(self) -> Result<Self::Ok> {
        if self.finished {
            Err(serde::ser::Error::custom("sequence already ended"))
        } else {
            // Mock the end of the sequence, change the state
            Ok(super::Value::Null) // Return a mock result
        }
    }
}

#[test]
fn test_end_function_success() {
    let seq = MockSerializeSeq::default();
    let result = seq.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), super::Value::Null);
}

#[test]
#[should_panic(expected = "sequence already ended")]
fn test_end_function_panic_on_double_end() {
    let mut seq = MockSerializeSeq::default();
    let _ = seq.end().unwrap(); // First call to end
    seq.finished = true; // Simulating that the sequence is finished
    let _ = seq.end(); // Second call to end should panic
}

