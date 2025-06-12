// Answer 0

#[test]
fn test_seq_access_new_with_valid_mutable_reference() {
    let scratch: Vec<u8> = Vec::new();
    let deserializer = Deserializer {
        read: &mut scratch,
        scratch,
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let mut de = deserializer;
    let _seq_access = SeqAccess::new(&mut de);
}

#[test]
fn test_seq_access_new_with_zero_remaining_depth() {
    let scratch: Vec<u8> = Vec::new();
    let deserializer = Deserializer {
        read: &mut scratch,
        scratch,
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let mut de = deserializer;
    let _seq_access = SeqAccess::new(&mut de);
}

#[test]
fn test_seq_access_new_with_maximum_remaining_depth() {
    let scratch: Vec<u8> = Vec::new();
    let deserializer = Deserializer {
        read: &mut scratch,
        scratch,
        remaining_depth: 255,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let mut de = deserializer;
    let _seq_access = SeqAccess::new(&mut de);
}

