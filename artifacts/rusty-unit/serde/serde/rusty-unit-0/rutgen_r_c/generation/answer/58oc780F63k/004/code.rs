// Answer 0

#[test]
fn test_serialize_socketaddr_v4_not_human_readable() {
    use serde::Serializer;
    use std::net::{SocketAddr, Ipv4Addr};

    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = &'static str;

        fn is_human_readable(&self) -> bool {
            false
        }

        fn serialize_newtype_variant<V>(&self, _: &'static str, _: u32, _: &'static str, _: &V) -> Result<Self::Ok, Self::Error>
        where
            V: Serialize,
        {
            Ok(())
        }

        // Other trait methods can be left unimplemented for this test.
    }

    let addr = SocketAddr::V4(std::net::SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080));
    let serializer = MockSerializer;

    let result = addr.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_socketaddr_v6_not_human_readable() {
    use serde::Serializer;
    use std::net::{SocketAddr, Ipv6Addr};

    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = &'static str;

        fn is_human_readable(&self) -> bool {
            false
        }

        fn serialize_newtype_variant<V>(&self, _: &'static str, _: u32, _: &'static str, _: &V) -> Result<Self::Ok, Self::Error>
        where
            V: Serialize,
        {
            Ok(())
        }

        // Other trait methods can be left unimplemented for this test.
    }

    let addr = SocketAddr::V6(std::net::SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 0, 0));
    let serializer = MockSerializer;

    let result = addr.serialize(serializer);
    assert!(result.is_ok());
}

