// Answer 0

#[test]
fn test_deserialize_human_readable_ipv4() {
    use crate::lib::*;
    use serde::de::IntoDeserializer;
    use std::net::SocketAddrV4;

    let input = "192.168.0.1:8080"; // a valid human-readable IPv4 address
    let deserializer = input.into_deserializer();
    
    let result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_string(), "192.168.0.1:8080");
}

#[test]
fn test_deserialize_human_readable_ipv6() {
    use crate::lib::*;
    use serde::de::IntoDeserializer;
    use std::net::SocketAddrV6;

    let input = "[2001:db8::1]:8080"; // a valid human-readable IPv6 address
    let deserializer = input.into_deserializer();
    
    let result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().to_string(), "[2001:db8::1]:8080");
}

#[should_panic]
fn test_deserialize_invalid_ip() {
    use crate::lib::*;
    use serde::de::IntoDeserializer;

    let input = "invalid_ip"; // invalid human-readable address
    let deserializer = input.into_deserializer();
    
    let _: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer).unwrap(); // should panic
}

#[test]
fn test_deserialize_empty_input() {
    use crate::lib::*;
    use serde::de::IntoDeserializer;

    let input = ""; // empty input
    let deserializer = input.into_deserializer();
    
    let result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
    
    assert!(result.is_err());
}

#[test]
fn test_deserialize_human_readable_without_port() {
    use crate::lib::*;
    use serde::de::IntoDeserializer;

    let input = "192.168.0.1"; // human-readable IPv4 address without port
    let deserializer = input.into_deserializer();
    
    let result: Result<SocketAddr, _> = SocketAddr::deserialize(deserializer);
    
    assert!(result.is_err());
}

