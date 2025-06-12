// Answer 0

#[test]
fn test_new_entry() {
    struct Entries<K, V> {
        // minimal structure for testing
        _phantom_key: std::marker::PhantomData<K>,
        _phantom_value: std::marker::PhantomData<V>,
    }
    
    struct OccupiedEntry<'a, T> {
        _phantom: std::marker::PhantomData<&'a T>,
    }

    struct Entry<'a, K, V> {
        entries: &'a mut Entries<K, V>,
        index: OccupiedEntry<'a, usize>,
    }

    impl<'a, K, V> Entry<'a, K, V> {
        pub(crate) fn new(entries: &'a mut Entries<K, V>, index: OccupiedEntry<'a, usize>) -> Self {
            Self { entries, index }
        }
    }

    // Create instances for testing
    let mut entries = Entries::<i32, i32> {
        _phantom_key: std::marker::PhantomData,
        _phantom_value: std::marker::PhantomData,
    };
    
    let index = OccupiedEntry::<usize> {
        _phantom: std::marker::PhantomData,
    };

    // Actual test
    let entry = Entry::new(&mut entries, index);
    assert!(!std::ptr::null::<Entries<i32, i32>>() == entry.entries as *const _);
}

