// Answer 0

#[test]
fn test_get_mut() {
    use crate::{IndexMapCore, Entries, HashValue};
    
    struct TestKey(i32);
    struct TestValue(i32);
    
    // Setup a dummy index map and entries for testing
    let mut entries = Entries::<TestKey, TestValue>::new();
    entries.push(Bucket {
        hash: HashValue::default(),
        key: TestKey(1),
        value: TestValue(10),
    });
    
    let mut index_map = IndexMapCore {
        entries: entries,
    };

    let mut indexed_entry = IndexedEntry::new(&mut index_map, 0);
    
    // Validate initial value
    assert_eq!(indexed_entry.get_mut().0, 10);
    
    // Modify the value through get_mut
    *indexed_entry.get_mut() = TestValue(20);
    
    // Validate the changed value
    assert_eq!(indexed_entry.get_mut().0, 20);
}

#[test]
#[should_panic]
fn test_get_mut_out_of_bounds() {
    use crate::{IndexMapCore, Entries, HashValue};
    
    struct TestKey(i32);
    struct TestValue(i32);
    
    // Setup a dummy index map and entries for testing
    let mut entries = Entries::<TestKey, TestValue>::new();
    entries.push(Bucket {
        hash: HashValue::default(),
        key: TestKey(1),
        value: TestValue(10),
    });
    
    let mut index_map = IndexMapCore {
        entries: entries,
    };

    let mut indexed_entry = IndexedEntry::new(&mut index_map, 1);
    
    // This should panic because the index is out of bounds
    let _ = indexed_entry.get_mut();
}

