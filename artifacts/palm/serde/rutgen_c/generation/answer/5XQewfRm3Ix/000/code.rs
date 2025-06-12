// Answer 0

#[test]
fn test_serialize_human_readable() {
    struct MockSerializer {
        human_readable: bool,
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        
        fn is_human_readable(&self) -> bool {
            self.human_readable
        }

        fn serialize_str(&self, _value: &str) -> Result<Self::Ok, Self::Error> {
            assert_eq!(_value, "101.102.103.104:65000");
            Ok(())
        }

        fn serialize_tuple(self, _len: usize) -> Result<MockTupleSerializer, Self::Error> {
            Err(())
        }
        
        fn collect_seq<T>(self, _items: T) -> Result<Self::Ok, Self::Error> {
            Err(())
        }

        fn collect_map<K, V, T>(self, _items: T) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
    }

    struct MockTupleSerializer;

    impl MockTupleSerializer {
        fn serialize_element<T>(&mut self, _: &T) -> Result<(), ()> {
            Err(())
        }

        fn end(self) -> Result<(), ()> {
            Err(())
        }
    }
    
    let socket_addr_v4 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(101, 102, 103, 104), 65000);
    let serializer = MockSerializer { human_readable: true };

    let result = socket_addr_v4.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_non_human_readable() {
    struct MockSerializer {
        human_readable: bool,
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        
        fn is_human_readable(&self) -> bool {
            self.human_readable
        }

        fn serialize_str(&self, _value: &str) -> Result<Self::Ok, Self::Error> {
            Err(())
        }

        fn serialize_tuple(self, _len: usize) -> Result<MockTupleSerializer, Self::Error> {
            Ok(MockTupleSerializer)
        }
        
        fn collect_seq<T>(self, _items: T) -> Result<Self::Ok, Self::Error> {
            Err(())
        }

        fn collect_map<K, V, T>(self, _items: T) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
    }

    struct MockTupleSerializer;

    impl MockTupleSerializer {
        fn serialize_element<T>(&mut self, _: &T) -> Result<(), ()> {
            Err(())
        }

        fn end(self) -> Result<(), ()> {
            Err(())
        }
    }

    let socket_addr_v4 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(101, 102, 103, 104), 65000);
    let serializer = MockSerializer { human_readable: false };

    let result = socket_addr_v4.serialize(serializer);
    assert!(result.is_ok());
}

