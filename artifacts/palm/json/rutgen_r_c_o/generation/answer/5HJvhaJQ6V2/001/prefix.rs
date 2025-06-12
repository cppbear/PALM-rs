// Answer 0

#[test]
fn test_serialize_bytes_empty() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[]); 
}

#[test]
fn test_serialize_bytes_one_byte() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[0]);
}

#[test]
fn test_serialize_bytes_two_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[1, 2]);
}

#[test]
fn test_serialize_bytes_three_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[3, 4, 5]);
}

#[test]
fn test_serialize_bytes_four_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[6, 7, 8, 9]);
}

#[test]
fn test_serialize_bytes_five_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[10, 11, 12, 13, 14]);
}

#[test]
fn test_serialize_bytes_six_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_serialize_bytes_seven_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[21, 22, 23, 24, 25, 26, 27]);
}

#[test]
fn test_serialize_bytes_eight_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[28, 29, 30, 31, 32, 33, 34, 35]);
}

#[test]
fn test_serialize_bytes_nine_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[36, 37, 38, 39, 40, 41, 42, 43, 44]);
}

#[test]
fn test_serialize_bytes_ten_bytes() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer: Vec::new(), formatter: CompactFormatter } };
    serializer.serialize_bytes(&[45, 46, 47, 48, 49, 50, 51, 52, 53, 54]);
}

