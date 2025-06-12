// Answer 0

#[test]
fn test_get_many_mut() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling()) // Simple placeholder for allocation
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut libraries: HashMap<String, usize, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };
    
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    libraries.insert("Library of Congress".to_string(), 1800);

    let [Some(a), Some(b)] = libraries.get_many_mut([
        "Athenæum",
        "Bodleian Library",
    ]) else { panic!() };

    assert_eq!(*a, 1807);
    assert_eq!(*b, 1602);
}

#[test]
fn test_get_many_mut_some_missing_keys() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut libraries: HashMap<String, usize, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);

    let got = libraries.get_many_mut([
        "Athenæum",
        "New York Public Library",
    ]);
    assert_eq!(
        got,
        [
            Some(&mut 1807),
            None
        ],
    );
}

#[test]
#[should_panic]
fn test_get_many_mut_duplicate_keys() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut libraries: HashMap<String, usize, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    libraries.insert("Athenæum".to_string(), 1807);

    let _got = libraries.get_many_mut([
        "Athenæum",
        "Athenæum",
    ]);
}

