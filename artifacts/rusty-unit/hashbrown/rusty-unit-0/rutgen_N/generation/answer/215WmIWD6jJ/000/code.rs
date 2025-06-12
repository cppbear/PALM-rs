// Answer 0

#[derive(Debug)]
struct RawTableInner;

impl RawTableInner {
    const NEW: Self = RawTableInner;
}

struct HashTable {
    table: RawTableInner,
    alloc: Global,
    marker: std::marker::PhantomData<()>,
}

struct Global;

impl Global {
    fn new() -> Self {
        Global
    }
}

impl HashTable {
    pub const fn new() -> Self {
        Self {
            table: RawTableInner::NEW,
            alloc: Global::new(),
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_hash_table_creation() {
    let ht = HashTable::new();
    assert_eq!(format!("{:?}", ht.table), format!("{:?}", RawTableInner::NEW));
}

