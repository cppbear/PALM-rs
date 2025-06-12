// Answer 0

#[test]
fn test_serialize_human_readable() {
    struct MockSerializer {
        is_human_readable: bool,
        output: String,
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        
        fn is_human_readable(&self) -> bool {
            self.is_human_readable
        }

        fn serialize_str(&mut self, value: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(value);
            Ok(())
        }

        fn collect_seq<T>(&mut self, _: &T) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_tuple(&mut self, _: usize) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let address = std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(1001, 1002, 1003, 1004,
                        1005, 1006, 1007, 1008), 65000, 0, 0);
    let mut serializer = MockSerializer { is_human_readable: true, output: String::new() };

    let result = address.serialize(&mut serializer);
    
    assert!(result.is_ok());
    assert_eq!(serializer.output, "[1001:1002:1003:1004:1005:1006:1007:1008%4294967295]:65000");
}

#[test]
fn test_serialize_binary() {
    struct MockSerializer {
        is_human_readable: bool,
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();

        fn is_human_readable(&self) -> bool {
            self.is_human_readable
        }

        fn serialize_str(&mut self, _: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn collect_seq<T>(&mut self, _: &T) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn serialize_tuple(&mut self, _: usize) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let address = std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(1001, 1002, 1003, 1004,
                        1005, 1006, 1007, 1008), 65000, 0, 0);
    let mut serializer = MockSerializer { is_human_readable: false };

    let result = address.serialize(&mut serializer);
    
    assert!(result.is_ok());
}

