// Answer 0

#[test]
fn test_from_key_found() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    let map: HashMap<&str, u32, BuildHasherDefault<DefaultHasher>> = [("a", 100), ("b", 200)].into();
    let key = "a";
    let entry = {
        let builder = RawEntryBuilder { map: &map };
        builder.from_key(&key)
    };
    assert_eq!(entry, Some((&"a", &100)));
}

#[test]
fn test_from_key_not_found() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    let map: HashMap<&str, u32, BuildHasherDefault<DefaultHasher>> = [("a", 100), ("b", 200)].into();
    let key = "c";
    let entry = {
        let builder = RawEntryBuilder { map: &map };
        builder.from_key(&key)
    };
    assert_eq!(entry, None);
}

#[test]
fn test_from_key_equivalent_type() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    let map: HashMap<String, u32, BuildHasherDefault<DefaultHasher>> = [("a".to_string(), 100), ("b".to_string(), 200)].into();
    let key = String::from("a");
    let entry = {
        let builder = RawEntryBuilder { map: &map };
        builder.from_key(&key)
    };
    assert_eq!(entry, Some((&"a", &100)));
}

