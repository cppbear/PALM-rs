// Answer 0

#[derive(Debug)]
struct MyStruct {
    ip: String,
    port: u16,
}

impl MyStruct {
    fn new(ip: &str, port: u16) -> Self {
        MyStruct {
            ip: ip.to_string(),
            port,
        }
    }

    fn ip(&self) -> &str {
        &self.ip
    }

    fn port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::ser::{Serializer, Serialize};

    struct MockSerializer {
        human_readable: bool,
        output: String,
    }

    impl MockSerializer {
        fn new(human_readable: bool) -> Self {
            MockSerializer {
                human_readable,
                output: String::new(),
            }
        }
    }

    impl Serializer for MockSerializer {
        type Ok = String;
        type Error = std::string::String;

        fn is_human_readable(&self) -> bool {
            self.human_readable
        }

        // Implement other required methods with basic functionality for testing
        fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(value);
            Ok(self.output.clone())
        }

        fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(&value.to_string());
            Ok(self.output.clone())
        }

        fn serialize_tuple(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
            // Not needed for this test
            Ok(self.output.clone())
        }
    }

    #[test]
    fn test_serialize_human_readable() {
        let my_struct = MyStruct::new("192.168.1.1", 8080);
        let serializer = MockSerializer::new(true);
        let result = my_struct.serialize(serializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "192.168.1.18080");
    }

    #[test]
    fn test_serialize_non_human_readable() {
        let my_struct = MyStruct::new("192.168.1.1", 8080);
        let serializer = MockSerializer::new(false);
        let result = my_struct.serialize(serializer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "192.168.1.18080");
    }
}

