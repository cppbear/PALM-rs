// Answer 0

#[test]
fn test_key_returns_correct_key() {
    use hashbrown::HashMap;

    struct VacantEntry<K> {
        key: K,
    }

    impl<K> VacantEntry<K> {
        fn key(&self) -> &K {
            &self.key
        }
    }

    let entry = VacantEntry { key: "poneyland" };
    assert_eq!(entry.key(), &"poneyland");
}

#[test]
fn test_key_with_different_type() {
    use hashbrown::HashMap;

    struct VacantEntry<K> {
        key: K,
    }

    impl<K> VacantEntry<K> {
        fn key(&self) -> &K {
            &self.key
        }
    }

    let entry = VacantEntry { key: 42 };
    assert_eq!(entry.key(), &42);
}

