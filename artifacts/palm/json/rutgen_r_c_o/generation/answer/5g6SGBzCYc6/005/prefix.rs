// Answer 0

#[test]
fn test_parse_ident_lowercase() {
    let mut deserializer = Deserializer {
        read: StrRead::new("abcde"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let ident = [b'a', b'b', b'c', b'd', b'e'];
    deserializer.parse_ident(&ident);
}

#[test]
fn test_parse_ident_uppercase() {
    let mut deserializer = Deserializer {
        read: StrRead::new("ABCDE"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let ident = [b'A', b'B', b'C', b'D', b'E'];
    deserializer.parse_ident(&ident);
}

#[test]
fn test_parse_ident_numeric() {
    let mut deserializer = Deserializer {
        read: StrRead::new("123"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let ident = [b'1', b'2', b'3'];
    deserializer.parse_ident(&ident);
}

#[test]
fn test_parse_ident_special_characters() {
    let mut deserializer = Deserializer {
        read: StrRead::new("!@#"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let ident = [b'!', b'@', b'#'];
    deserializer.parse_ident(&ident);
}

#[test]
fn test_parse_ident_boundary_values() {
    let mut deserializer = Deserializer {
        read: StrRead::new("\x00\x01\x02"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let ident = [b'\x00', b'\x01', b'\x02'];
    deserializer.parse_ident(&ident);
}

