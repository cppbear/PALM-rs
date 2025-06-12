// Answer 0

#[test]
fn test_get_many_unchecked_mut_inner() {
    struct MockAllocator;
    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut hashmap: HashMap<i32, i32, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    // Inserting values into the hashmap
    hashmap.insert(1, 10);
    hashmap.insert(2, 20);
    hashmap.insert(3, 30);

    // Keys to fetch
    let keys = [&1, &2, &3];

    // Calling the unsafe method
    let results: [Option<&mut (i32, i32)>; 3] = unsafe { hashmap.get_many_unchecked_mut_inner(keys) };

    // Assertion to check if we got the expected mutable references
    assert_eq!(results[0].as_ref().map(|&(_, v)| v), Some(10));
    assert_eq!(results[1].as_ref().map(|&(_, v)| v), Some(20));
    assert_eq!(results[2].as_ref().map(|&(_, v)| v), Some(30));
}

#[test]
#[should_panic]
fn test_get_many_unchecked_mut_inner_panic() {
    struct MockAllocator;
    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut hashmap: HashMap<i32, i32, DefaultHashBuilder, MockAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: MockAllocator,
            marker: PhantomData,
        },
    };

    // Inserting values into the hashmap
    hashmap.insert(1, 10);
    
    // Keys that will cause a panic because one doesn't exist
    let keys = [&1, &2]; // Key 2 hasn't been inserted

    // Attempting to call the unsafe method
    let _results: [Option<&mut (i32, i32)>; 2] = unsafe { hashmap.get_many_unchecked_mut_inner(keys) };
}

