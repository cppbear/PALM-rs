// Answer 0

#[test]
fn test_serialize_human_readable() {
    use serde::Serializer;
    use std::net::SocketAddrV4;

    struct MockSerializer {
        human_readable: bool,
        output: String,
        error: Option<String>,
    }

    impl MockSerializer {
        fn new(human_readable: bool) -> Self {
            MockSerializer {
                human_readable,
                output: String::new(),
                error: None,
            }
        }
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = Self;
        type SerializeTuple = Self;

        fn is_human_readable(&self) -> bool {
            self.human_readable
        }

        fn collect_seq<T>(self, _seq: T) -> Result<Self::Ok, Self::Error>
        where
            T: serde::ser::SerializeSeq,
        {
            Ok(())
        }

        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Ok(self)
        }

        fn serialize_str(&mut self, value: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(value);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            Ok(())
        }
    }

    let addr: SocketAddrV4 = "101.102.103.104:65000".parse().unwrap();
    let mut serializer = MockSerializer::new(true);
    let result = addr.serialize(&mut serializer);
    assert!(result.is_ok());
    assert_eq!(serializer.output, "101.102.103.104:65000");
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_serialize_panic_condition() {
    use serde::Serializer;
    use std::net::SocketAddrV4;

    struct MockSerializer {
        human_readable: bool,
        output: String,
        error: Option<String>,
    }

    impl MockSerializer {
        fn new(human_readable: bool) -> Self {
            MockSerializer {
                human_readable,
                output: String::new(),
                error: None,
            }
        }
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = Self;
        type SerializeTuple = Self;

        fn is_human_readable(&self) -> bool {
            self.human_readable
        }

        fn collect_seq<T>(self, _seq: T) -> Result<Self::Ok, Self::Error>
        where
            T: serde::ser::SerializeSeq,
        {
            Ok(())
        }

        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Ok(self)
        }

        fn serialize_str(&mut self, value: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(value);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            Ok(())
        }
    }

    let addr: SocketAddrV4 = "101.102.103.104:65000".parse().unwrap();
    let mut serializer = MockSerializer::new(false);
    let result = addr.serialize(&mut serializer);
    // This assertion will panic because we expect a human-readable output
    assert_eq!(serializer.output, "101.102.103.104:65000");
}

