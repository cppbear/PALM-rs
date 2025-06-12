// Answer 0

#[test]
fn test_serialize_v4_human_readable_min() {
    let addr = net::SocketAddr::V4(Ipv4Addr::new(0, 0, 0, 1), 1);
    let serializer = MySerializer::new(true); // Assuming MySerializer implements Serializer
    addr.serialize(serializer);
}

#[test]
fn test_serialize_v4_human_readable_mid() {
    let addr = net::SocketAddr::V4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_v4_human_readable_max() {
    let addr = net::SocketAddr::V4(Ipv4Addr::new(255, 255, 255, 255), 65535);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

#[test]
fn test_serialize_v4_human_readable_random() {
    let addr = net::SocketAddr::V4(Ipv4Addr::new(192, 168, 1, 1), 54321);
    let serializer = MySerializer::new(true);
    addr.serialize(serializer);
}

