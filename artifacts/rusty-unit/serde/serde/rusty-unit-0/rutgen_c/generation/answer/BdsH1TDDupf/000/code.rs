// Answer 0

#[test]
fn test_ipv4_addr_serialize_human_readable() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = String;
        type Error = ();
        
        fn is_human_readable(&self) -> bool {
            true
        }
        
        fn serialize_str(&self, value: &str) -> Result<Self::Ok, Self::Error> {
            Ok(value.to_string())
        }
        
        fn collect_seq<T>(&self, _seq: &T) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn collect_map<T>(&self, _map: &T) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_tuple(&self, _len: usize) -> Result<Self::Ok, Self::Error> { unimplemented!() }
    }
    
    let ip = net::Ipv4Addr::new(101, 102, 103, 104);
    let serializer = DummySerializer;
    let result = ip.serialize(serializer).unwrap();
    assert_eq!(result, "101.102.103.104");
}

#[test]
fn test_ipv4_addr_serialize_binary() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = Vec<u8>;
        type Error = ();
        
        fn is_human_readable(&self) -> bool {
            false
        }
        
        fn serialize_str(&self, _value: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        
        fn collect_seq<T>(&self, _seq: &T) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn collect_map<T>(&self, _map: &T) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_tuple(&self, _len: usize) -> Result<Self::Ok, Self::Error> { unimplemented!() }
    }
    
    let ip = net::Ipv4Addr::new(101, 102, 103, 104);
    let serializer = DummySerializer;
    let result = ip.serialize(serializer).unwrap();
    assert_eq!(result, vec![101, 102, 103, 104]);
}

