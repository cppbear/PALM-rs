// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
        Err(())
    }

    fn serialize_str(self, _: &str) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::String))
    }
}

#[derive(Debug)]
struct Unsupported;

#[test]
fn test_serialize_str_error() {
    let serializer = TestSerializer;
    let result = serializer.serialize_str("test string");
    assert_eq!(result, Err(()));
}

