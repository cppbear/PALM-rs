// Answer 0

#[test]
fn test_get_many_mut_found_unique() {
    struct MyTable {
        data: HashMap<u64, i32>,
    }

    impl MyTable {
        fn new() -> Self {
            MyTable {
                data: HashMap::new(),
            }
        }

        fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<*mut i32>; N] {
            hashes.map(|hash| {
                self.data.iter_mut()
                    .find_map(|(k, v)| if eq(hash as usize, v) { Some(v as *mut _) } else { None })
            })
        }
    }

    let mut table = MyTable::new();
    table.data.insert(1, 10);
    table.data.insert(2, 20);
    table.data.insert(3, 30);

    let result: [Option<&mut i32>; 3] = table.get_many_mut([1, 2, 3], |i, v| *v == (i as i32 + 9));
    assert_eq!(result, [Some(&mut 10), Some(&mut 20), Some(&mut 30)]);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_found_duplicate() {
    struct MyTable {
        data: HashMap<u64, i32>,
    }

    impl MyTable {
        fn new() -> Self {
            MyTable {
                data: HashMap::new(),
            }
        }

        fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<*mut i32>; N] {
            hashes.map(|hash| {
                self.data.iter_mut()
                    .find_map(|(k, v)| if eq(hash as usize, v) { Some(v as *mut _) } else { None })
            })
        }
    }

    let mut table = MyTable::new();
    table.data.insert(1, 10);
    table.data.insert(2, 20);
    table.data.insert(3, 20); // Duplicate value with key 2

    let _result: [Option<&mut i32>; 3] = table.get_many_mut([1, 2, 3], |i, v| *v == 20);
}

#[test]
fn test_get_many_mut_not_found() {
    struct MyTable {
        data: HashMap<u64, i32>,
    }

    impl MyTable {
        fn new() -> Self {
            MyTable {
                data: HashMap::new(),
            }
        }

        fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &i32) -> bool,
        ) -> [Option<*mut i32>; N] {
            hashes.map(|hash| {
                self.data.iter_mut()
                    .find_map(|(k, v)| if eq(hash as usize, v) { Some(v as *mut _) } else { None })
            })
        }
    }

    let mut table = MyTable::new();
    table.data.insert(1, 10);
    table.data.insert(2, 20);

    let result: [Option<&mut i32>; 3] = table.get_many_mut([1, 3, 4], |i, v| *v == (i as i32 + 9));
    assert_eq!(result, [Some(&mut 10), None, None]);
}

