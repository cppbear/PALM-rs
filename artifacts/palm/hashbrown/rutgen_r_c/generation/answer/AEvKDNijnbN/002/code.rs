// Answer 0

#[test]
fn test_build_hashes_inner_empty_array() {
    struct MyKey;
    struct MyHasher;

    impl Hash for MyKey {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, _: &Self) -> bool {
            true
        }
    }

    struct MyMap {
        hash_builder: MyHasher,
    }

    impl MyMap {
        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + Equivalent<MyKey> + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = make_hash::<Q, MyHasher>(&self.hash_builder, ks[i]);
            }
            hashes
        }
    }

    fn make_hash<Q, S>(_hash_builder: &S, _: &Q) -> u64 {
        42 // Arbitrary hash value for testing
    }

    let my_map = MyMap { hash_builder: MyHasher };
    let keys: [&MyKey; 0] = []; // Empty array
    let hashes = my_map.build_hashes_inner(keys);
    
    assert_eq!(hashes, []);
}

#[test]
fn test_build_hashes_inner_one_element() {
    struct MyKey;
    struct MyHasher;

    impl Hash for MyKey {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, _: &Self) -> bool {
            true
        }
    }

    struct MyMap {
        hash_builder: MyHasher,
    }

    impl MyMap {
        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + Equivalent<MyKey> + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = make_hash::<Q, MyHasher>(&self.hash_builder, ks[i]);
            }
            hashes
        }
    }

    fn make_hash<Q, S>(_hash_builder: &S, _: &Q) -> u64 {
        42 // Arbitrary hash value for testing
    }

    let my_map = MyMap { hash_builder: MyHasher };
    let keys: [&MyKey; 1] = [&MyKey]; // Array with one element
    let hashes = my_map.build_hashes_inner(keys);
    
    assert_eq!(hashes, [42]);
} 

#[test]
fn test_build_hashes_inner_multiple_elements() {
    struct MyKey;
    struct MyHasher;

    impl Hash for MyKey {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<MyKey> for MyKey {
        fn equivalent(&self, _: &Self) -> bool {
            true
        }
    }

    struct MyMap {
        hash_builder: MyHasher,
    }

    impl MyMap {
        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + Equivalent<MyKey> + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = make_hash::<Q, MyHasher>(&self.hash_builder, ks[i]);
            }
            hashes
        }
    }

    fn make_hash<Q, S>(_hash_builder: &S, _: &Q) -> u64 {
        42 // Arbitrary hash value for testing
    }

    let my_map = MyMap { hash_builder: MyHasher };
    let keys: [&MyKey; 3] = [&MyKey, &MyKey, &MyKey]; // Array with three elements
    let hashes = my_map.build_hashes_inner(keys);
    
    assert_eq!(hashes, [42, 42, 42]);
}

