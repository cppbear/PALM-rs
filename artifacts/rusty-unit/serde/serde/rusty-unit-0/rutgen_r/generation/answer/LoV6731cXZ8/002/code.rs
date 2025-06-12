// Answer 0

#[derive(Debug)]
struct TestSerializer {
    human_readable: bool,
}

impl TestSerializer {
    fn new(human_readable: bool) -> Self {
        TestSerializer { human_readable }
    }
}

impl Serializer for TestSerializer {
    type Ok = ();
    type Error = std::io::Error;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        if value.len() > 39 {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "String is too long"))
        } else {
            Ok(())
        }
    }

    fn is_human_readable(&self) -> bool {
        self.human_readable
    }
}

#[test]
fn test_serialize_human_readable_bounded_length() {
    let serializer = TestSerializer::new(true);
    let octets = "1001:1002:1003:1004:1005:1006:1007:1008"; // 39 characters

    let result = serialize(&octets, serializer);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "String is too long")]
fn test_serialize_human_readable_exceeds_bounded_length() {
    let serializer = TestSerializer::new(true);
    let octets = "1001:1002:1003:1004:1005:1006:1007:1008:1009"; // 40 characters

    let _ = serialize(&octets, serializer); // Expected to panic
}

#[test]
fn test_serialize_non_human_readable() {
    let serializer = TestSerializer::new(false);
    let octets = "1001:1002:1003:1004:1005:1006:1007:1008"; // Valid input

    let result = serialize(&octets, serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_non_human_readable_longer_than_39() {
    let serializer = TestSerializer::new(false);
    let octets = "1001:1002:1003:1004:1005:1006:1007:1008:1009"; // Valid but longer than 39

    let result = serialize(&octets, serializer);
    assert!(result.is_ok());
}

