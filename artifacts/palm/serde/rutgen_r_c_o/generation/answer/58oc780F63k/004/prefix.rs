// Answer 0

#[test]
fn test_serialize_ipv4() {
    use std::net::{SocketAddr, Ipv4Addr};
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn is_human_readable(&self) -> bool { false }
        fn serialize_newtype_variant(&self, _: &str, _: u32, _: &str, _: &dyn Serialize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let serializer = MockSerializer;

    let addr = SocketAddr::V4(SocketAddr::from(Ipv4Addr::new(192, 168, 1, 1).into()));
    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_zero() {
    use std::net::{SocketAddr, Ipv4Addr};
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn is_human_readable(&self) -> bool { false }
        fn serialize_newtype_variant(&self, _: &str, _: u32, _: &str, _: &dyn Serialize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let serializer = MockSerializer;

    let addr = SocketAddr::V4(SocketAddr::from(Ipv4Addr::new(0, 0, 0, 0).into()));
    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_broadcast() {
    use std::net::{SocketAddr, Ipv4Addr};
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn is_human_readable(&self) -> bool { false }
        fn serialize_newtype_variant(&self, _: &str, _: u32, _: &str, _: &dyn Serialize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let serializer = MockSerializer;

    let addr = SocketAddr::V4(SocketAddr::from(Ipv4Addr::new(255, 255, 255, 255).into()));
    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_private() {
    use std::net::{SocketAddr, Ipv4Addr};
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn is_human_readable(&self) -> bool { false }
        fn serialize_newtype_variant(&self, _: &str, _: u32, _: &str, _: &dyn Serialize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let serializer = MockSerializer;

    let addr = SocketAddr::V4(SocketAddr::from(Ipv4Addr::new(10, 0, 0, 1).into()));
    addr.serialize(serializer);
}

#[test]
fn test_serialize_ipv4_loopback() {
    use std::net::{SocketAddr, Ipv4Addr};
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn is_human_readable(&self) -> bool { false }
        fn serialize_newtype_variant(&self, _: &str, _: u32, _: &str, _: &dyn Serialize) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let serializer = MockSerializer;

    let addr = SocketAddr::V4(SocketAddr::from(Ipv4Addr::new(127, 0, 0, 1).into()));
    addr.serialize(serializer);
}

