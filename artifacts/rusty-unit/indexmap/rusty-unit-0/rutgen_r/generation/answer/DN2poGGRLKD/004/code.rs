// Answer 0

#[test]
fn test_increment_indices_full_sweep() {
    struct TestStruct {
        entries: Vec<Entry>,
        indices: Vec<usize>,
    }

    struct Entry {
        hash: usize,
    }

    fn update_index(indices: &mut Vec<usize>, hash: usize, old_index: usize, new_index: usize) {
        // Dummy implementation to mimic behavior for testing
        if let Some(pos) = indices.iter().position(|&x| x == old_index) {
            indices[pos] = new_index;
        }
    }

    let mut my_struct = TestStruct {
        entries: vec![Entry { hash: 1 }, Entry { hash: 2 }, Entry { hash: 3 }, Entry { hash: 4 }, Entry { hash: 5 }, Entry { hash: 6 }],
        indices: vec![0, 1, 2, 3, 4, 5, 6], // capacity is 7
    };

    // Start and end indices to increment
    let start = 1;
    let end = 5; // shifted_entries.len() == 4 which is > 7/2

    my_struct.increment_indices(start, end);

    assert_eq!(my_struct.indices, vec![0, 2, 3, 4, 5, 6, 7]); // Expect indices to be shifted correctly
}

#[test]
#[should_panic]
fn test_increment_indices_panic_out_of_bounds() {
    struct TestStruct {
        entries: Vec<Entry>,
        indices: Vec<usize>,
    }

    struct Entry {
        hash: usize,
    }

    let my_struct = TestStruct {
        entries: vec![Entry { hash: 1 }, Entry { hash: 2 }],
        indices: vec![0, 1, 2, 3],
    };

    // This will panic because the end index is out of bounds
    my_struct.increment_indices(0, 3);
}

