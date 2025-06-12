// Answer 0

#[derive(Debug)]
struct MockSerializer {
    human_readable: bool,
}

impl MockSerializer {
    fn new(human_readable: bool) -> Self {
        MockSerializer { human_readable }
    }

    fn is_human_readable(&self) -> bool {
        self.human_readable
    }

    fn serialize(&self, addr: &str) -> Result<&str, &'static str> {
        if self.human_readable {
            Ok(addr)
        } else {
            Err("Not human readable")
        }
    }

    fn serialize_newtype_variant(&self, _type: &str, _index: usize, variant: &str, addr: &str) -> Result<&str, &'static str> {
        if self.human_readable {
            Ok(format!("{}({})", variant, addr).as_str())
        } else {
            Err("Not human readable")
        }
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            match *self {
                IpAddr::V4(ref a) => serializer.serialize(a),
                IpAddr::V6(ref a) => serializer.serialize(a),
            }
        } else {
            match *self {
                IpAddr::V4(ref a) => serializer.serialize_newtype_variant("IpAddr", 0, "V4", a),
                IpAddr::V6(ref a) => serializer.serialize_newtype_variant("IpAddr", 1, "V6", a),
            }
        }
    }
}

#[test]
fn test_serialize_ipaddr_v4_human_readable() {
    let serializer = MockSerializer::new(true);
    let ip_addr = IpAddr::V4("192.168.0.1".to_string());
    
    let result = ip_addr.serialize(serializer);

    assert_eq!(result.unwrap(), "192.168.0.1");
}

#[test]
fn test_serialize_ipaddr_v6_human_readable() {
    let serializer = MockSerializer::new(true);
    let ip_addr = IpAddr::V6("::1".to_string());

    let result = ip_addr.serialize(serializer);

    assert_eq!(result.unwrap(), "::1");
}

