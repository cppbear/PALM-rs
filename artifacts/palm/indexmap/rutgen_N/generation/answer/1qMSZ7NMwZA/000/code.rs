// Answer 0

#[test]
fn test_len_empty() {
    struct MapSlice {
        entries: Vec<(i32, i32)>,
    }

    impl MapSlice {
        pub const fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let map_slice = MapSlice { entries: Vec::new() };
    assert_eq!(map_slice.len(), 0);
}

#[test]
fn test_len_single_entry() {
    struct MapSlice {
        entries: Vec<(i32, i32)>,
    }

    impl MapSlice {
        pub const fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let map_slice = MapSlice { entries: vec![(1, 100)] };
    assert_eq!(map_slice.len(), 1);
}

#[test]
fn test_len_multiple_entries() {
    struct MapSlice {
        entries: Vec<(i32, i32)>,
    }

    impl MapSlice {
        pub const fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let map_slice = MapSlice { entries: vec![(1, 100), (2, 200), (3, 300)] };
    assert_eq!(map_slice.len(), 3);
}

