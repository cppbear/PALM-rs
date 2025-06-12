// Answer 0

#[test]
fn test_deserialize_ip_addr_human_readable() {
    use crate::lib::net::IpAddr;
    use serde_json::from_str;

    let json = "\"127.0.0.1\""; // a human-readable IP
    let result: Result<IpAddr, _> = from_str(json);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IpAddr::V4("127.0.0.1".parse().unwrap()));
}

#[test]
fn test_deserialize_ip_addr_binary_v4() {
    use crate::lib::net::IpAddr;
    use bincode::deserialize;

    let bytes: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // binary for "0.0.0.1"
    let result: Result<IpAddr, _> = deserialize(&bytes);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IpAddr::V4("0.0.0.1".parse().unwrap()));
}

#[test]
fn test_deserialize_ip_addr_binary_v6() {
    use crate::lib::net::IpAddr;
    use bincode::deserialize;

    let bytes: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // binary for "::1"
    let result: Result<IpAddr, _> = deserialize(&bytes);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), IpAddr::V6("::1".parse().unwrap()));
}

#[should_panic(expected = "invalid length")]
fn test_deserialize_ip_addr_invalid_length() {
    use crate::lib::net::IpAddr;
    use bincode::deserialize;

    let bytes: Vec<u8> = vec![0]; // invalid byte array
    let _: Result<IpAddr, _> = deserialize(&bytes).unwrap();
}

