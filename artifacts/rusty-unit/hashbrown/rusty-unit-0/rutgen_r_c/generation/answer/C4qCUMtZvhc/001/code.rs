// Answer 0

#[test]
fn test_entry_ref_or_default_vacant_entry() {
    use crate::hashbrown::HashMap;

    struct SimpleHasher;

    impl BuildHasher for SimpleHasher {
        type Hasher = std::hash::rustc_hash::RustcHash;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    // Create a HashMap with the custom hasher that returns Option<u32>
    let mut map: HashMap<String, Option<u32, SimpleHasher>> = HashMap::new();

    // Ensure that for a nonexistent key, the or_default method creates a default entry
    let entry_ref = map.entry_ref("poneyland");
    let value = entry_ref.or_default();
    assert_eq!(value, &mut None);
    assert_eq!(map["poneyland"], None);

    // Insert another key and verify that or_default works on an occupied entry
    map.insert("horseland".to_string(), Some(3));
    let occupied_value = map.entry_ref("horseland").or_default();
    assert_eq!(occupied_value, &mut Some(3));
}

