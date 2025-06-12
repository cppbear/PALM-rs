// Answer 0

#[test]
fn test_serialize_tuple_struct_zero_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let _ = serializer.serialize_tuple_struct("test", 0);
}

#[test]
fn test_serialize_tuple_struct_one_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let _ = serializer.serialize_tuple_struct("test", 1);
}

#[test]
fn test_serialize_tuple_struct_two_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let _ = serializer.serialize_tuple_struct("test", 2);
}

#[test]
fn test_serialize_tuple_struct_three_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let _ = serializer.serialize_tuple_struct("test", 3);
}

#[test]
fn test_serialize_tuple_struct_large_length() {
    let writer = Vec::new();
    let formatter = CompactFormatter;
    let mut serializer = MapKeySerializer {
        ser: &mut Serializer {
            writer,
            formatter,
        },
    };
    let _ = serializer.serialize_tuple_struct("test", 1000);
}

