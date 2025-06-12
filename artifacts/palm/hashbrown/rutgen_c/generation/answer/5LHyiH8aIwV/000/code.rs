// Answer 0

#[test]
fn test_get_many_key_value_unchecked_mut() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }
    
    let mut map: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };
    
    map.insert("Bodleian Library".to_string(), 1602).unwrap();
    map.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691).unwrap();
    
    let keys = [
        "Bodleian Library",
        "Herzogin-Anna-Amalia-Bibliothek",
    ];
    let got = unsafe { map.get_many_key_value_unchecked_mut(&keys) };
    
    assert_eq!(
        got,
        [
            Some((&"Bodleian Library".to_string(), &mut 1602)),
            Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
        ],
    );
}

#[test]
#[should_panic]
fn test_get_many_key_value_unchecked_mut_with_missing_key() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }
    
    let mut map: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };
    
    map.insert("Bodleian Library".to_string(), 1602).unwrap();
    
    let keys = [
        "Bodleian Library",
        "Nonexistent Library",
    ];
    let _ = unsafe { map.get_many_key_value_unchecked_mut(&keys) };
}

