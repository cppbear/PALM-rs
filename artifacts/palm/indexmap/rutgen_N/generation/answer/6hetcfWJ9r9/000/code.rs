// Answer 0

#[derive(Debug)]
struct Entry<K> {
    key: K,
}

impl<K> Entry<K> {
    pub fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }
    
    pub fn insert_key(&mut self, key: K) -> K {
        std::mem::replace(self.key_mut(), key)
    }
}

#[test]
fn test_insert_key_replaces_old_key() {
    let mut entry = Entry { key: "old_key" };
    let old_key = entry.insert_key("new_key");
    assert_eq!(old_key, "old_key");
    assert_eq!(entry.key, "new_key");
}

#[test]
fn test_insert_key_same_key() {
    let mut entry = Entry { key: "same_key" };
    let old_key = entry.insert_key("same_key");
    assert_eq!(old_key, "same_key");
    assert_eq!(entry.key, "same_key");
}

