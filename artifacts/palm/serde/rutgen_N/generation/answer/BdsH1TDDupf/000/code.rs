// Answer 0

#[derive(Debug)]
struct MyOctets {
    octets: [u8; 4],
}

impl MyOctets {
    fn octets(&self) -> &[u8; 4] {
        &self.octets
    }
}

impl serde::ser::Serialize for MyOctets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            const MAX_LEN: usize = 15;
            debug_assert_eq!(MAX_LEN, "101.102.103.104".len());
            let mut buf = [b'.'; MAX_LEN];
            let mut written = format_u8(self.octets()[0], &mut buf);
            for oct in &self.octets()[1..] {
                written += format_u8(*oct, &mut buf[written + 1..]) + 1;
            }
            let buf = unsafe { std::str::from_utf8_unchecked(&buf[..written]) };
            serializer.serialize_str(buf)
        } else {
            self.octets().serialize(serializer)
        }
    }
}

fn format_u8(octet: u8, buf: &mut [u8]) -> usize {
    let str_rep = octet.to_string();
    let bytes = str_rep.as_bytes();
    let len = bytes.len();
    buf[..len].copy_from_slice(bytes);
    len
}

#[test]
fn test_serialize_human_readable() {
    let octets = MyOctets { octets: [101, 102, 103, 104] };
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    serializer.serialize_str("Serialized: ").unwrap();
    let result = octets.serialize(&mut serializer);
    assert!(result.is_ok());
} 

#[test]
fn test_serialize_binary() {
    let octets = MyOctets { octets: [101, 102, 103, 104] };
    let mut serializer = serde_json::Serializer::with_format(std::io::stdout(), serde_json::ser::CompactFormat);
    let result = octets.serialize(&mut serializer);
    assert!(result.is_ok());
} 

#[should_panic]
#[test]
fn test_serialize_invalid_length() {
    let octets = MyOctets { octets: [255, 255, 255, 255] }; // Example that could theoretically result in long output
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    octets.serialize(&mut serializer).unwrap();
}

