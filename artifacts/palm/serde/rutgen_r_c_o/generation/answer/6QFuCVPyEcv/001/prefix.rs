// Answer 0

#[test]
fn test_serialize_human_readable_valid() {
    let addr = std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::new(1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008),
        65000,
        0,
        0,
    );
    let mut buffer = [0u8; 58];
    let serializer = MyHumanReadableSerializer::new(&mut buffer);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_human_readable_edge_case() {
    let addr = std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
        65000,
        0,
        0,
    );
    let mut buffer = [0u8; 58];
    let serializer = MyHumanReadableSerializer::new(&mut buffer);
    addr.serialize(serializer);
}

#[test]
#[should_panic]
fn test_serialize_human_readable_invalid_length() {
    let addr = std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::new(1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008),
        65000,
        0,
        0,
    );
    let mut buffer = [0u8; 10]; // Intentionally too small
    let serializer = MyHumanReadableSerializer::new(&mut buffer);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_binary_format() {
    let addr = std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::new(1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008),
        65000,
        0,
        0,
    );
    let serializer = MyBinarySerializer::new();
    addr.serialize(serializer);
}

#[test]
fn test_serialize_edge_case_binary_format() {
    let addr = std::net::SocketAddrV6::new(
        std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
        1,
        0,
        0,
    );
    let serializer = MyBinarySerializer::new();
    addr.serialize(serializer);
}

