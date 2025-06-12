// Answer 0

#[derive(Debug)]
struct MockSerializer {
    human_readable: bool,
}

impl MockSerializer {
    fn new(human_readable: bool) -> Self {
        Self { human_readable }
    }

    fn is_human_readable(&self) -> bool {
        self.human_readable
    }

    fn serialize_str(&self, value: &str) -> Result<&str, &'static str> {
        Ok(value)
    }
}

#[derive(Debug)]
struct IpAddr {
    octets: [u8; 4],
}

impl IpAddr {
    fn octets(&self) -> &[u8] {
        &self.octets
    }

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: MockSerializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 15;
            let mut buf = [b'.'; MAX_LEN];
            let mut written = format_u8(self.octets()[0], &mut buf);
            for &oct in &self.octets()[1..] {
                written += format_u8(oct, &mut buf[written + 1..]) + 1;
            }
            let buf = unsafe { std::str::from_utf8_unchecked(&buf[..written]) };
            serializer.serialize_str(buf)
        } else {
            // Here you would handle the binary serialization
            unimplemented!()
        }
    }
}

fn format_u8(val: u8, buf: &mut [u8]) -> usize {
    val.to_string().as_bytes().iter().enumerate().take_while(|&(i, &b)| b != b'\0').for_each(|(i, &b)| buf[i] = b);
    val.to_string().len()
}

#[test]
fn test_serialize_human_readable_with_valid_ip() {
    let serializer = MockSerializer::new(true);
    let ip = IpAddr { octets: [101, 102, 103, 104] };
    
    let result = ip.serialize(serializer);
    assert_eq!(result, Ok("101.102.103.104"));
}

#[test]
fn test_serialize_empty_octets() {
    let serializer = MockSerializer::new(true);
    let ip = IpAddr { octets: [0, 0, 0, 0] };
    
    let result = ip.serialize(serializer);
    assert_eq!(result, Ok("0.0.0.0"));
}

#[test]
#[should_panic]
fn test_serialize_with_invalid_octets() {
    let serializer = MockSerializer::new(true);
    let ip = IpAddr { octets: [255, 255, 255, 255] }; // This would fail if we assume out-of-bound access (simulated).

    let _ = ip.serialize(serializer); // This test is expected to panic due to out-of-bounds access in octets.
}

