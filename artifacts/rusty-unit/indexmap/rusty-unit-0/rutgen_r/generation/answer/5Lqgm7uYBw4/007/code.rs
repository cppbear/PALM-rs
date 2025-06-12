// Answer 0

#[test]
fn test_swap_indices_success() {
    use std::collections::HashMap;

    struct Entry {
        hash: Hash,
        value: i32,
    }

    struct Hash(u64);

    struct TestStruct {
        entries: Vec<Entry>,
        indices: HashMap<u64, usize>,
    }

    impl TestStruct {
        fn new(entries: Vec<Entry>, indices: HashMap<u64, usize>) -> Self {
            TestStruct { entries, indices }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_mut(&self.entries[a].hash.0)
                .zip(self.indices.get_mut(&self.entries[b].hash.0)) {
                Some((ref_a, ref_b)) => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let entry1 = Entry { hash: Hash(1), value: 10 };
    let entry2 = Entry { hash: Hash(2), value: 20 };
    let entry3 = Entry { hash: Hash(3), value: 30 };
    
    let mut entries = vec![entry1, entry2, entry3];
    let mut indices = HashMap::new();
    indices.insert(1, 0);
    indices.insert(2, 1);
    indices.insert(3, 2);

    let mut test_struct = TestStruct::new(entries, indices);
    
    test_struct.swap_indices(0, 1);
    
    assert_eq!(test_struct.entries[0].value, 20);
    assert_eq!(test_struct.entries[1].value, 10);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_indices_not_found() {
    use std::collections::HashMap;

    struct Entry {
        hash: Hash,
        value: i32,
    }

    struct Hash(u64);

    struct TestStruct {
        entries: Vec<Entry>,
        indices: HashMap<u64, usize>,
    }

    impl TestStruct {
        fn new(entries: Vec<Entry>, indices: HashMap<u64, usize>) -> Self {
            TestStruct { entries, indices }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_mut(&self.entries[a].hash.0)
                .zip(self.indices.get_mut(&self.entries[b].hash.0)) {
                Some((ref_a, ref_b)) => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let entry1 = Entry { hash: Hash(1), value: 10 };
    let entry2 = Entry { hash: Hash(2), value: 20 };
    
    let mut entries = vec![entry1, entry2];
    let mut indices = HashMap::new();
    indices.insert(1, 0);

    let mut test_struct = TestStruct::new(entries, indices);
    
    test_struct.swap_indices(0, 1); // This should panic since index 2 is not found
}

