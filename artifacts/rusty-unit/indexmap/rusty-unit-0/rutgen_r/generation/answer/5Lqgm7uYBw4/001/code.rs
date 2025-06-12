// Answer 0

#[test]
fn test_swap_indices_equal_and_in_bounds() {
    struct Entry {
        hash: HashWrapper,
    }

    struct HashWrapper {
        value: usize,
    }

    struct TestMap {
        entries: Vec<Entry>,
        indices: IndexMapWrapper,
    }

    struct IndexMapWrapper {
        data: Vec<usize>,
    }

    impl IndexMapWrapper {
        fn get_many_mut<F>(
            &self,
            keys: [usize; 2],
            f: F,
        ) -> [Option<&mut usize>; 2]
        where
            F: Fn(usize, &usize) -> bool,
        {
            let mut result = [None, None];
            for (i, &key) in keys.iter().enumerate() {
                if key < self.data.len() && f(i, &self.data[key]) {
                    result[i] = Some(&mut self.data[key]);
                }
            }
            result
        }
    }

    impl TestMap {
        fn new(entries: Vec<Entry>, indices: IndexMapWrapper) -> Self {
            TestMap { entries, indices }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.value, self.entries[b].hash.value],
                move |i, &x| if i == 0 { x == a } else { x == b },
            ) {
                [Some(ref_a), Some(ref_b)] => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let entry1 = Entry { hash: HashWrapper { value: 0 } };
    let entry2 = Entry { hash: HashWrapper { value: 1 } };
    let indices = IndexMapWrapper { data: vec![0, 1] };
    let mut test_map = TestMap::new(vec![entry1, entry2], indices);

    test_map.swap_indices(0, 0);
    assert_eq!(test_map.entries[0].hash.value, 0);
    assert_eq!(test_map.entries[1].hash.value, 1);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds() {
    struct Entry {
        hash: HashWrapper,
    }

    struct HashWrapper {
        value: usize,
    }

    struct TestMap {
        entries: Vec<Entry>,
        indices: IndexMapWrapper,
    }

    struct IndexMapWrapper {
        data: Vec<usize>,
    }

    impl IndexMapWrapper {
        fn get_many_mut<F>(
            &self,
            keys: [usize; 2],
            f: F,
        ) -> [Option<&mut usize>; 2]
        where
            F: Fn(usize, &usize) -> bool,
        {
            let mut result = [None, None];
            for (i, &key) in keys.iter().enumerate() {
                if key < self.data.len() && f(i, &self.data[key]) {
                    result[i] = Some(&mut self.data[key]);
                }
            }
            result
        }
    }

    impl TestMap {
        fn new(entries: Vec<Entry>, indices: IndexMapWrapper) -> Self {
            TestMap { entries, indices }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            if a == b && a < self.entries.len() {
                return;
            }

            match self.indices.get_many_mut(
                [self.entries[a].hash.value, self.entries[b].hash.value],
                move |i, &x| if i == 0 { x == a } else { x == b },
            ) {
                [Some(ref_a), Some(ref_b)] => {
                    std::mem::swap(ref_a, ref_b);
                    self.entries.swap(a, b);
                }
                _ => panic!("indices not found"),
            }
        }
    }

    let entry1 = Entry { hash: HashWrapper { value: 0 } };
    let entry2 = Entry { hash: HashWrapper { value: 1 } };
    let indices = IndexMapWrapper { data: vec![0] }; // Only one index available
    let mut test_map = TestMap::new(vec![entry1, entry2], indices);

    test_map.swap_indices(0, 1); // This should panic due to out-of-bounds access
}

