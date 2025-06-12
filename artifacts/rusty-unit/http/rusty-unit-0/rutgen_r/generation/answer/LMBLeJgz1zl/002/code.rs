// Answer 0

#[test]
fn test_insert_occupied_with_valid_index() {
    struct Entry<T> {
        value: T,
        links: Option<Links>,
    }

    struct Links {
        next: usize,
    }

    struct BucketMap<T> {
        entries: Vec<Entry<T>>,
    }

    impl<T> BucketMap<T> {
        fn insert_occupied(&mut self, index: usize, value: T) -> T {
            if let Some(links) = self.entries[index].links {
                self.remove_all_extra_values(links.next);
            }

            let entry = &mut self.entries[index];
            std::mem::replace(&mut entry.value, value)
        }

        fn remove_all_extra_values(&mut self, _next: usize) {
            // Dummy implementation, assuming it's defined somewhere else
        }
    }

    let mut map = BucketMap {
        entries: vec![
            Entry { value: 1, links: None },
            Entry { value: 2, links: Some(Links { next: 1 }) },
            Entry { value: 3, links: Some(Links { next: 0 }) },
        ],
    };

    let returned_value = map.insert_occupied(1, 5);
    assert_eq!(returned_value, 2);
    assert_eq!(map.entries[1].value, 5);
}

#[should_panic]
#[test]
fn test_insert_occupied_with_invalid_index() {
    struct Entry<T> {
        value: T,
        links: Option<Links>,
    }

    struct Links {
        next: usize,
    }

    struct BucketMap<T> {
        entries: Vec<Entry<T>>,
    }

    impl<T> BucketMap<T> {
        fn insert_occupied(&mut self, index: usize, value: T) -> T {
            if let Some(links) = self.entries[index].links {
                self.remove_all_extra_values(links.next);
            }

            let entry = &mut self.entries[index];
            std::mem::replace(&mut entry.value, value)
        }

        fn remove_all_extra_values(&mut self, _next: usize) {
            // Dummy implementation, assuming it's defined somewhere else
        }
    }

    let mut map = BucketMap {
        entries: vec![
            Entry { value: 1, links: None },
            Entry { value: 2, links: Some(Links { next: 1 }) },
        ],
    };

    // This should panic because index 3 is out of bounds
    let _ = map.insert_occupied(3, 5);
}

