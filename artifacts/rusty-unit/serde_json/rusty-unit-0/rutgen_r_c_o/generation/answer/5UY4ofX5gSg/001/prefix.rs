// Answer 0

#[test]
fn test_serialize_char_null() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter {} };
    let map_serializer = MapKeySerializer { ser: &mut serializer };
    map_serializer.serialize_char('\u{0000}');
}

#[test]
fn test_serialize_char_ascii() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter {} };
    let map_serializer = MapKeySerializer { ser: &mut serializer };
    map_serializer.serialize_char('A');
}

#[test]
fn test_serialize_char_unicode() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter {} };
    let map_serializer = MapKeySerializer { ser: &mut serializer };
    map_serializer.serialize_char('\u{1F600}'); // 😀
}

#[test]
fn test_serialize_char_high_surrogate() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter {} };
    let map_serializer = MapKeySerializer { ser: &mut serializer };
    map_serializer.serialize_char('\u{D83D}'); // High surrogate
}

#[test]
fn test_serialize_char_low_surrogate() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter {} };
    let map_serializer = MapKeySerializer { ser: &mut serializer };
    map_serializer.serialize_char('\u{DC00}'); // Low surrogate
}

#[test]
fn test_serialize_char_last_unicode() {
    let mut serializer = Serializer { writer: Vec::new(), formatter: CompactFormatter {} };
    let map_serializer = MapKeySerializer { ser: &mut serializer };
    map_serializer.serialize_char('\u{10FFFF}');
}

