// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(&self, _unsupported: Unsupported) -> Result<(), ()> {
        Err(())
    }
}

trait Serializer {
    type Ok;
    type Error;

    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
}

impl Serializer for TestSerializer {
    type Ok = ();
    type Error = ();

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.bad_type(Unsupported::Optional)
    }
}

struct Unsupported;

impl Unsupported {
    const Optional: Self = Unsupported;
}

#[test]
fn test_serialize_none_returns_error() {
    let serializer = TestSerializer;
    let result = serializer.serialize_none();
    assert_eq!(result, Err(()));
}

