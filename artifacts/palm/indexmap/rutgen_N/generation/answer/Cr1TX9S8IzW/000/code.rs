// Answer 0

#[test]
fn test_key_returns_correct_key() {
    struct Entry<K> {
        key: K,
    }

    impl<K> Entry<K> {
        pub fn new(key: K) -> Self {
            Entry { key }
        }

        pub fn key(&self) -> &K {
            &self.key
        }
    }

    let entry = Entry::new("test_key");
    assert_eq!(entry.key(), &"test_key");
}

#[test]
fn test_key_with_different_type() {
    struct Entry<K> {
        key: K,
    }

    impl<K> Entry<K> {
        pub fn new(key: K) -> Self {
            Entry { key }
        }

        pub fn key(&self) -> &K {
            &self.key
        }
    }

    let entry = Entry::new(42);
    assert_eq!(entry.key(), &42);
}

#[test]
fn test_key_with_string() {
    struct Entry<K> {
        key: K,
    }

    impl<K> Entry<K> {
        pub fn new(key: K) -> Self {
            Entry { key }
        }

        pub fn key(&self) -> &K {
            &self.key
        }
    }

    let entry = Entry::new(String::from("key_string"));
    assert_eq!(entry.key(), &String::from("key_string"));
}

