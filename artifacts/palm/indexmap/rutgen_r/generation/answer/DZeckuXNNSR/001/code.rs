// Answer 0

fn test_update_index_existing_entry() {
    #[derive(Default)]
    struct Indices {
        data: Vec<usize>,
    }

    impl Indices {
        fn find_mut<F>(&mut self, hash: usize, f: F) -> Option<&mut usize>
        where
            F: FnMut(&usize) -> bool,
        {
            self.data.iter_mut().find(f)
        }
    }

    #[derive(Default)]
    struct HashValue {
        value: usize,
    }

    impl HashValue {
        fn get(&self) -> usize {
            self.value
        }
    }

    let mut table = Indices { data: vec![1, 2, 3] };
    let hash = HashValue { value: 1 }; // Set to a value that has an index in `data`
    let old = 1; // Existing index
    let new = 99;

    update_index(&mut table, hash, old, new);
    assert_eq!(table.data, vec![99, 2, 3]);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_update_index_non_existent_entry() {
    #[derive(Default)]
    struct Indices {
        data: Vec<usize>,
    }

    impl Indices {
        fn find_mut<F>(&mut self, hash: usize, f: F) -> Option<&mut usize>
        where
            F: FnMut(&usize) -> bool,
        {
            self.data.iter_mut().find(f)
        }
    }

    #[derive(Default)]
    struct HashValue {
        value: usize,
    }

    impl HashValue {
        fn get(&self) -> usize {
            self.value
        }
    }

    let mut table = Indices { data: vec![2, 3] };
    let hash = HashValue { value: 1 }; // Set to a value that has no corresponding index in `data`
    let old = 1; // Non-existing index
    let new = 99;

    update_index(&mut table, hash, old, new);
}

fn update_index(table: &mut Indices, hash: HashValue, old: usize, new: usize) {
    let index = table
        .find_mut(hash.get(), move |&i| i == old)
        .expect("index not found");
    *index = new;
}

