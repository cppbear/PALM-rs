// Answer 0

fn test_get_disjoint_mut_ok() {
    struct MyMap {
        data: indexmap::IndexMap<i32, char>,
    }

    impl MyMap {
        fn new() -> Self {
            let data = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
            MyMap { data }
        }

        fn get_disjoint_mut_values<const N: usize>(&mut self, keys: [&i32; N]) -> [Option<&mut char>; N] {
            self.data.get_disjoint_mut(keys)
        }
    }

    let mut map = MyMap::new();
    let result = map.get_disjoint_mut_values([&1, &2]);
    assert_eq!(result, [Some(&mut 'a'), Some(&mut 'c')]);
}

#[should_panic(expected = "duplicate keys found")]
fn test_get_disjoint_mut_overlapping_indices() {
    struct MyMap {
        data: indexmap::IndexMap<i32, char>,
    }

    impl MyMap {
        fn new() -> Self {
            let data = indexmap::IndexMap::from([(1, 'a'), (2, 'b'), (3, 'c')]);
            MyMap { data }
        }

        fn get_disjoint_mut_values<const N: usize>(&mut self, keys: [&i32; N]) -> [Option<&mut char>; N] {
            self.data.get_disjoint_mut(keys)
        }
    }

    let mut map = MyMap::new();
    let _ = map.get_disjoint_mut_values([&1, &1]);
}

#[should_panic(expected = "Internal error: indices should never be OOB as we got them from get_index_of")]
fn test_get_disjoint_mut_index_out_of_bounds() {
    struct MyMap {
        data: indexmap::IndexMap<i32, char>,
    }

    impl MyMap {
        fn new() -> Self {
            let data = indexmap::IndexMap::from([(1, 'a'), (2, 'b')]);
            MyMap { data }
        }

        fn get_disjoint_mut_values<const N: usize>(&mut self, keys: [&i32; N]) -> [Option<&mut char>; N] {
            self.data.get_disjoint_mut(keys)
        }
    }

    let mut map = MyMap::new();
    let _ = map.get_disjoint_mut_values([&1, &3]);
}

