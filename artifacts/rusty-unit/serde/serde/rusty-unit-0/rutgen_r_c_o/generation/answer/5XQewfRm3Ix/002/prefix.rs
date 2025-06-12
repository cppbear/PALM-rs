// Answer 0

#[test]
fn test_serialize_human_readable_with_valid_ip_and_port() {
    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 1, 1), 8080);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_human_readable_with_lower_bound_ip_and_non_default_port() {
    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 1), 8081);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_human_readable_with_upper_bound_ip_and_low_port() {
    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(255, 255, 255, 255), 1);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_human_readable_with_middle_ip_and_high_port() {
    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(100, 100, 100, 100), 65000);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_human_readable_with_different_ip_and_port() {
    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(10, 0, 0, 2), 3030);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
#[should_panic]
fn test_serialize_human_readable_with_panic_condition() {
    let addr1 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(10, 0, 0, 2), 8080);
    let addr2 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(10, 0, 0, 2), 8080); // same IP and port
    let serializer = MySerializer::new(true);
    
    assert_ne!(addr1.ip(), addr2.ip()); // This won't panic
    addr1.serialize(serializer);
}

