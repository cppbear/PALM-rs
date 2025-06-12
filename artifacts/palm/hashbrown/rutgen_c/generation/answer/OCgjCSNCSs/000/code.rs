// Answer 0

#[test]
fn test_insert_with_hasher() {
    use crate::hash_map::{HashMap, RawVacantEntryMut};
    use core::hash::{BuildHasher, Hash};
    use core::mem::MaybeUninit;
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "a";
    let hash_builder = map.hasher().clone();
    
    fn make_hasher<K, S>(hash_builder: &S) -> impl Fn(&K) -> u64 + '_
    where
        K: Hash + ?Sized,
        S: BuildHasher,
    {
        move |key: &K| {
            use core::hash::Hasher;
            let mut state = hash_builder.build_hasher();
            key.hash(&mut state);
            state.finish()
        }
    }

    let hash = make_hasher(&hash_builder)(&key);

    match map.raw_entry_mut().from_hash(hash, |q| q == &key) {
        RawEntryMut::Occupied(_) => panic!(),
        RawEntryMut::Vacant(v) => {
            let (k, v) = v.insert_with_hasher(hash, key, 100, make_hasher(&hash_builder));
            assert_eq!(k, &mut "a");
            assert_eq!(v, &mut 100);
        }
    }

    map.extend([("b", 200), ("c", 300), ("d", 400), ("e", 500), ("f", 600)]);
    assert_eq!(map[&"a"], 100);
}

