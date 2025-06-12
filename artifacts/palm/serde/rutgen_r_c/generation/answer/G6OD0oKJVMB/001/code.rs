// Answer 0

#[test]
fn test_deserialize_ipaddr_human_readable() {
    use std::io::{BufReader, Cursor};
    use serde_json::Deserializer;
    use crate::lib::net::IpAddr;

    // Prepare a readable deserializer
    let json_data = r#""192.168.1.1""#; // Valid IPv4 address in a human-readable format
    let reader = Cursor::new(json_data);
    let deserializer = Deserializer::from_reader(reader);

    // Ensure the deserialization works as expected for a valid IPv4 address
    let result: Result<IpAddr, _> = IpAddr::deserialize(deserializer);

    assert_eq!(result.is_ok(), true); // Expecting successful deserialization
    assert_eq!(result.unwrap(), IpAddr::V4("192.168.1.1".parse().unwrap())); // Check if the IP address is correctly parsed
}

#[test]
fn test_deserialize_ipaddr_invalid_human_readable() {
    use std::io::{BufReader, Cursor};
    use serde_json::Deserializer;
    use crate::lib::net::IpAddr;

    // Prepare a deserializer with invalid IP address
    let json_data = r#""invalid_ip""#; // Invalid IP address
    let reader = Cursor::new(json_data);
    let deserializer = Deserializer::from_reader(reader);

    // Ensure panics happen for invalid human-readable IP address
    let result: Result<IpAddr, _> = IpAddr::deserialize(deserializer);
    
    assert!(result.is_err()); // Expecting an error during deserialization
}

#[test]
fn test_deserialize_ipaddr_v6() {
    use std::io::{BufReader, Cursor};
    use serde_json::Deserializer;
    use crate::lib::net::IpAddr;

    // Prepare a readable deserializer for IPv6
    let json_data = r#""2001:0db8:85a3:0000:0000:8a2e:0370:7334""#; // Valid IPv6 address
    let reader = Cursor::new(json_data);
    let deserializer = Deserializer::from_reader(reader);

    // Ensure the deserialization works as expected for a valid IPv6 address
    let result: Result<IpAddr, _> = IpAddr::deserialize(deserializer);

    assert_eq!(result.is_ok(), true); // Expecting successful deserialization
    assert_eq!(result.unwrap(), IpAddr::V6("2001:0db8:85a3:0000:0000:8a2e:0370:7334".parse().unwrap())); // Check if the IP address is correctly parsed
}

