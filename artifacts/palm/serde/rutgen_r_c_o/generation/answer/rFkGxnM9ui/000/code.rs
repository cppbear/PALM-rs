// Answer 0

#[derive(Debug)]
struct FakeSerializer;

impl FakeSerializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
        Err(())
    }
}

#[derive(Debug)]
struct Unsupported {
    kind: &'static str,
}

impl Unsupported {
    fn new(kind: &'static str) -> Self {
        Unsupported { kind }
    }
}

#[test]
fn test_serialize_u8() {
    let serializer = FakeSerializer;
    let result = serializer.serialize_u8(5);
    let expected: Result<(), ()> = Err(());
    assert_eq!(result, expected);
}

impl FakeSerializer {
    fn serialize_u8(self, _: u8) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::new("Integer")))
    }
}

