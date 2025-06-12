// Answer 0

#[test]
fn test_new_seq_access() {
    struct MockDeserializer;

    let mut deserializer = Deserializer {
        read: MockDeserializer,
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let seq_access = SeqAccess::new(&mut deserializer);
    
    assert!(!seq_access.first);
}

