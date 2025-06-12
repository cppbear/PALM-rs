// Answer 0

#[derive(Default)]
struct DummyMap<K, V> {
    // Example fields to simulate a map structure
    key: K,
    value: V,
}

struct RawEntryBuilder<'a, K, V, S> {
    map: &'a DummyMap<K, V>,
}

impl<K, V, S> DummyMap<K, V> {
    fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S> {
        RawEntryBuilder { map: self }
    }
}

#[test]
fn test_raw_entry_v1() {
    let dummy_map: DummyMap<i32, String> = DummyMap {
        key: 42,
        value: "Hello".to_string(),
    };
    
    let entry_builder = dummy_map.raw_entry_v1();
    
    assert_eq!(entry_builder.map.key, 42);
    assert_eq!(entry_builder.map.value, "Hello");
}

