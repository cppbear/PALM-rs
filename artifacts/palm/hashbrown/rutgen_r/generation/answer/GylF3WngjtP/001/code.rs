// Answer 0

#[test]
fn test_get_many_unchecked_mut_with_valid_hashes() {
    struct Dummy {
        value: i32,
    }

    struct DummyStore {
        items: Vec<Dummy>,
    }

    impl DummyStore {
        pub unsafe fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &Dummy) -> bool,
        ) -> [Option<*mut Dummy>; N] {
            hashes.map(|hash| {
                let index = (hash as usize) % self.items.len();
                if eq(index, &self.items[index]) {
                    Some(&mut self.items[index] as *mut Dummy)
                } else {
                    None
                }
            })
        }

        pub fn new() -> Self {
            Self { items: vec![Dummy { value: 1 }, Dummy { value: 2 }, Dummy { value: 3 }] }
        }
    }

    let mut store = DummyStore::new();
    let hashes = [0, 1, 2];
    
    let result = unsafe {
        store.get_many_unchecked_mut(hashes, |index, _| index < store.items.len())
    };

    assert_eq!(result.len(), 3);
    assert!(result[0].is_some());
    assert!(result[1].is_some());
    assert!(result[2].is_some());
    assert_eq!(result[0].unwrap().value, 1);
    assert_eq!(result[1].unwrap().value, 2);
    assert_eq!(result[2].unwrap().value, 3);
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_with_out_of_bounds_hash() {
    struct Dummy {
        value: i32,
    }

    struct DummyStore {
        items: Vec<Dummy>,
    }

    impl DummyStore {
        pub unsafe fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &Dummy) -> bool,
        ) -> [Option<*mut Dummy>; N] {
            hashes.map(|hash| {
                let index = (hash as usize) % self.items.len();
                if eq(index, &self.items[index]) {
                    Some(&mut self.items[index] as *mut Dummy)
                } else {
                    None
                }
            })
        }

        pub fn new() -> Self {
            Self { items: vec![Dummy { value: 1 }, Dummy { value: 2 }, Dummy { value: 3 }] }
        }
    }

    let mut store = DummyStore::new();
    let hashes = [0, 1, 3]; // The last index is out of bounds

    unsafe {
        store.get_many_unchecked_mut(hashes, |index, _| index < store.items.len());
    }
}

#[test]
fn test_get_many_unchecked_mut_with_empty_store() {
    struct Dummy {
        value: i32,
    }

    struct DummyStore {
        items: Vec<Dummy>,
    }

    impl DummyStore {
        pub unsafe fn get_many_mut_pointers<const N: usize>(
            &mut self,
            hashes: [u64; N],
            mut eq: impl FnMut(usize, &Dummy) -> bool,
        ) -> [Option<*mut Dummy>; N] {
            hashes.map(|hash| {
                let index = (hash as usize) % self.items.len();
                if eq(index, &self.items[index]) {
                    Some(&mut self.items[index] as *mut Dummy)
                } else {
                    None
                }
            })
        }

        pub fn new() -> Self {
            Self { items: Vec::new() }
        }
    }

    let mut store = DummyStore::new();
    let hashes = [0, 1, 2]; // No items in the store

    let result = unsafe {
        store.get_many_unchecked_mut(hashes, |index, _| false)
    };

    assert_eq!(result.len(), 3);
    assert!(result.iter().all(|option| option.is_none()));
}

