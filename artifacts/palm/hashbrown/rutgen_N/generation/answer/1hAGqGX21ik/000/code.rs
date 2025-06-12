// Answer 0

#[derive(Debug)]
struct DummyAllocator;

struct HashTable<A> {
    table: RawTableInner,
    alloc: A,
    marker: std::marker::PhantomData<*const ()>,
}

impl DummyAllocator {
    fn new() -> Self {
        DummyAllocator
    }
}

#[derive(Debug)]
struct RawTableInner;

impl RawTableInner {
    const NEW: Self = RawTableInner;
}

#[test]
fn test_new_in_with_dummy_allocator() {
    let allocator = DummyAllocator::new();
    let hash_table: HashTable<DummyAllocator> = HashTable {
        table: RawTableInner::NEW,
        alloc: allocator,
        marker: std::marker::PhantomData,
    };
    assert_eq!(hash_table.table, RawTableInner::NEW);
}

#[test]
fn test_new_in_does_not_panic() {
    let allocator = DummyAllocator::new();
    let hash_table = HashTable {
        table: RawTableInner::NEW,
        alloc: allocator,
        marker: std::marker::PhantomData,
    };
    assert!(std::mem::size_of_val(&hash_table) > 0);
}

