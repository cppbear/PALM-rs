// Answer 0

#[test]
fn test_last_with_non_empty_entries() {
    struct Bucket<'a, K, V> {
        key: K,
        value: V,
        _phantom: std::marker::PhantomData<&'a ()>,
    }

    struct Entries<'a, K, V> {
        entries: Vec<Bucket<'a, K, V>>,
    }

    impl<'a, K, V> Entries<'a, K, V> {
        fn last(&self) -> Option<(&K, &V)> {
            self.entries.last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let bucket1 = Bucket {
        key: "key1",
        value: 1,
        _phantom: std::marker::PhantomData,
    };
    let bucket2 = Bucket {
        key: "key2",
        value: 2,
        _phantom: std::marker::PhantomData,
    };
    let entries = Entries {
        entries: vec![bucket1, bucket2],
    };

    assert_eq!(entries.last(), Some((&"key2", &2)));
}

#[test]
fn test_last_with_empty_entries() {
    struct Bucket<'a, K, V> {
        key: K,
        value: V,
        _phantom: std::marker::PhantomData<&'a ()>,
    }

    struct Entries<'a, K, V> {
        entries: Vec<Bucket<'a, K, V>>,
    }

    impl<'a, K, V> Entries<'a, K, V> {
        fn last(&self) -> Option<(&K, &V)> {
            self.entries.last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let entries: Entries<&str, i32> = Entries { entries: Vec::new() };

    assert_eq!(entries.last(), None);
}

#[test]
#[should_panic]
fn test_last_with_uninitialized() {
    struct Bucket<'a, K, V> {
        key: K,
        value: V,
        _phantom: std::marker::PhantomData<&'a ()>,
    }

    struct Entries<'a, K, V> {
        entries: Vec<Bucket<'a, K, V>>,
    }

    impl<'a, K, V> Entries<'a, K, V> {
        fn last(&self) -> Option<(&K, &V)> {
            self.entries.last().map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    // Here we cause an intentional panic by trying to access a nonexistent entry.
    // This represents a misuse case, thus we expect it to panic.
    let entries: Entries<i32, i32> = Entries { entries: vec![] };
    let result = entries.last();
    assert!(result.is_none());
}

