// Answer 0

#[test]
fn test_ipv4_addr_serialize_human_readable() {
    use crate::ser::{Serializer, Serialize};
    use std::net::Ipv4Addr;

    struct DummySerializer {
        is_human_readable: bool,
        output: String,
    }

    impl DummySerializer {
        fn new(is_human_readable: bool) -> Self {
            Self {
                is_human_readable,
                output: String::new(),
            }
        }
    }

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = &'static str;

        fn is_human_readable(&self) -> bool {
            self.is_human_readable
        }

        fn serialize_str(&mut self, value: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(value);
            Ok(())
        }

        fn collect_seq<T>(&mut self, _values: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err("not implemented")
        }

        fn collect_map<T>(&mut self, _values: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err("not implemented")
        }
    }

    let ip = Ipv4Addr::new(192, 168, 1, 1); // typically used for local network
    let mut serializer = DummySerializer::new(true);
    let result = ip.serialize(&mut serializer);

    assert!(result.is_ok());
    assert_eq!(serializer.output, "192.168.1.1");
}

#[test]
fn test_ipv4_addr_serialize_human_readable_boundaries() {
    use crate::ser::{Serializer, Serialize};
    use std::net::Ipv4Addr;

    struct DummySerializer {
        is_human_readable: bool,
        output: String,
    }

    impl DummySerializer {
        fn new(is_human_readable: bool) -> Self {
            Self {
                is_human_readable,
                output: String::new(),
            }
        }
    }

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = &'static str;

        fn is_human_readable(&self) -> bool {
            self.is_human_readable
        }

        fn serialize_str(&mut self, value: &str) -> Result<Self::Ok, Self::Error> {
            self.output.push_str(value);
            Ok(())
        }

        fn collect_seq<T>(&mut self, _values: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err("not implemented")
        }

        fn collect_map<T>(&mut self, _values: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err("not implemented")
        }
    }

    let ip = Ipv4Addr::new(255, 255, 255, 255); // upper boundary scenario
    let mut serializer = DummySerializer::new(true);
    let result = ip.serialize(&mut serializer);

    assert!(result.is_ok());
    assert_eq!(serializer.output, "255.255.255.255");
}

#[test]
#[should_panic]
fn test_ipv4_addr_serialize_should_panic_on_empty_octets() {
    use crate::ser::{Serializer, Serialize};
    use std::net::Ipv4Addr;

    struct DummySerializer {
        is_human_readable: bool,
    }

    impl DummySerializer {
        fn new(is_human_readable: bool) -> Self {
            Self {
                is_human_readable,
            }
        }
    }

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = &'static str;

        fn is_human_readable(&self) -> bool {
            self.is_human_readable
        }

        fn serialize_str(&mut self, _value: &str) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }

        fn collect_seq<T>(&mut self, _values: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err("not implemented")
        }

        fn collect_map<T>(&mut self, _values: &T) -> Result<Self::Ok, Self::Error>
        where
            T: Serialize,
        {
            Err("not implemented")
        }
    }

    let ip = Ipv4Addr::new(0, 0, 0, 0); // will trigger a panic if empty slice check is made
    let mut serializer = DummySerializer::new(true);
    let _ = ip.serialize(&mut serializer);
}

