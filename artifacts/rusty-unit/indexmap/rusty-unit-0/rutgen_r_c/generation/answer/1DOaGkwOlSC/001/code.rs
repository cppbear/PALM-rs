// Answer 0

#[test]
fn test_get_index_of_found() {
    struct KeyEquivalent(usize);
    impl Equivalent<usize> for KeyEquivalent {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }

    let mut map: IndexMapCore<usize, &'static str> = IndexMapCore::new();
    map.push_entry(HashValue(1), 1, "one");
    map.push_entry(HashValue(2), 2, "two");
    map.push_entry(HashValue(3), 3, "three");

    let index = map.get_index_of(HashValue(2), &KeyEquivalent(2));
    assert_eq!(index, Some(1));
}

#[test]
fn test_get_index_of_not_found() {
    struct KeyEquivalent(usize);
    impl Equivalent<usize> for KeyEquivalent {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }

    let mut map: IndexMapCore<usize, &'static str> = IndexMapCore::new();
    map.push_entry(HashValue(1), 1, "one");
    map.push_entry(HashValue(2), 2, "two");

    let index = map.get_index_of(HashValue(3), &KeyEquivalent(3));
    assert_eq!(index, None);
}

#[test]
fn test_get_index_of_empty_map() {
    struct KeyEquivalent(usize);
    impl Equivalent<usize> for KeyEquivalent {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }

    let map: IndexMapCore<usize, &'static str> = IndexMapCore::new();
    let index = map.get_index_of(HashValue(1), &KeyEquivalent(1));
    assert_eq!(index, None);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_index_of_out_of_bounds() {
    struct KeyEquivalent(usize);
    impl Equivalent<usize> for KeyEquivalent {
        fn equivalent(&self, other: &usize) -> bool {
            self.0 == *other
        }
    }

    let mut map: IndexMapCore<usize, &'static str> = IndexMapCore::new();
    map.push_entry(HashValue(1), 1, "one");
    map.push_entry(HashValue(2), 2, "two");
    map.split_off(3);
}

