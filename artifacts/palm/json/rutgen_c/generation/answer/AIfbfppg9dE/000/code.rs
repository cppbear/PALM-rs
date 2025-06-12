// Answer 0

#[test]
fn test_map_access_new() {
    struct MockDeserializer;

    let mut deserializer = Deserializer {
        read: MockDeserializer,
        scratch: Vec::new(),
        remaining_depth: 0,
    };

    let map_access = MapAccess::new(&mut deserializer);
    assert!(map_access.first);
}

