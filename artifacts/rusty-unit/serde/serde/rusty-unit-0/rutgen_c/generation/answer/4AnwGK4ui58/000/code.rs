// Answer 0

#[test]
fn test_deserialize_socket_address_v4() {
    use crate::lib::net::SocketAddrV4;
    use serde_json;

    let json_data = r#""192.168.1.1:8080""#;
    let result: Result<SocketAddrV4, _> = serde_json::from_str(json_data);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_socket_address_v6() {
    use crate::lib::net::SocketAddrV6;
    use serde_json;

    let json_data = r#""[::1]:8080""#;
    let result: Result<SocketAddrV6, _> = serde_json::from_str(json_data);
    assert!(result.is_ok());
}

#[should_panic]
fn test_deserialize_invalid_socket_address() {
    use crate::lib::net::SocketAddr;
    use serde_json;

    let json_data = r#""invalid address""#;
    let _: Result<SocketAddr, _> = serde_json::from_str(json_data).unwrap();
}

