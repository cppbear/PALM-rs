// Answer 0

#[test]
fn test_deserialize_ipv4_socket_address() {
    use serde::de::value::MapDeserializer;
    use std::collections::HashMap;
    use crate::lib::net::{SocketAddrV4, SocketAddr, Ipv4Addr};

    let map: HashMap<&str, u16> = HashMap::new();
    let deserializer = MapDeserializer::new(map.into_iter());

    let result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
    assert!(result.is_ok());
    if let Ok(addr) = result {
        assert_eq!(addr, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
    }
}

#[test]
fn test_deserialize_ipv6_socket_address() {
    use serde::de::value::MapDeserializer;
    use std::collections::HashMap;
    use crate::lib::net::{SocketAddrV6, SocketAddr, Ipv6Addr};

    let map: HashMap<&str, u16> = HashMap::new();
    let deserializer = MapDeserializer::new(map.into_iter());

    let result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
    assert!(result.is_ok());
    if let Ok(addr) = result {
        assert_eq!(addr, SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0)));
    }
}

#[should_panic(expected = "unknown variant")]
#[test]
fn test_deserialize_invalid_socket_address() {
    use crate::lib::net::SocketAddr;
    use serde::de::value::MapDeserializer;
    use std::collections::HashMap;

    let map: HashMap<&str, u16> = HashMap::new(); // No valid input for deserialization
    let deserializer = MapDeserializer::new(map.into_iter());

    let _result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
}

