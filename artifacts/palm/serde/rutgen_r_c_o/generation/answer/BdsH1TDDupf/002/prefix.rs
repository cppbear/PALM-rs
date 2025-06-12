// Answer 0

#[test]
fn test_serialize_ipv4_addr_human_readable() {
    use std::net::Ipv4Addr;
    use serde_json::Serializer;

    let addr = Ipv4Addr::new(192, 168, 1, 1);
    let serializer = Serializer::new(Vec::new()).with_human_readable();

    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_addr_human_readable_min_value() {
    use std::net::Ipv4Addr;
    use serde_json::Serializer;

    let addr = Ipv4Addr::new(0, 0, 0, 1);
    let serializer = Serializer::new(Vec::new()).with_human_readable();

    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_addr_human_readable_max_first_octet() {
    use std::net::Ipv4Addr;
    use serde_json::Serializer;

    let addr = Ipv4Addr::new(255, 0, 0, 1);
    let serializer = Serializer::new(Vec::new()).with_human_readable();

    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_addr_human_readable_max_second_octet() {
    use std::net::Ipv4Addr;
    use serde_json::Serializer;

    let addr = Ipv4Addr::new(192, 255, 0, 1);
    let serializer = Serializer::new(Vec::new()).with_human_readable();

    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_addr_human_readable_multiple_octets() {
    use std::net::Ipv4Addr;
    use serde_json::Serializer;

    let addr = Ipv4Addr::new(101, 102, 103, 104);
    let serializer = Serializer::new(Vec::new()).with_human_readable();

    addr.serialize(serializer);
}

