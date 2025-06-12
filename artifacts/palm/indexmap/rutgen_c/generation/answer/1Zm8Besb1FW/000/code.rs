// Answer 0

#[test]
fn test_insert_sorted_with_sorted_keys() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue(2), key: 2, value: "two".to_string() },
        Bucket { hash: HashValue(3), key: 3, value: "three".to_string() },
    ];
    
    let r_mut = RefMut {
        indices: &mut [],
        entries: &mut entries,
    };
    
    let vacant_entry = VacantEntry {
        map: r_mut,
        hash: HashValue(4),
        key: 2,
    };
    
    let (index, value_ref) = vacant_entry.insert_sorted("new value".to_string());
    
    assert_eq!(index, 1);
    assert_eq!(value_ref, &mut entries[index].value);
    assert_eq!(entries[index].value, "new value".to_string());
}

#[test]
fn test_insert_sorted_with_unsorted_keys() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    let mut entries = vec![
        Bucket { hash: HashValue(3), key: 3, value: "three".to_string() },
        Bucket { hash: HashValue(1), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue(2), key: 2, value: "two".to_string() },
    ];
    
    let r_mut = RefMut {
        indices: &mut [],
        entries: &mut entries,
    };
    
    let vacant_entry = VacantEntry {
        map: r_mut,
        hash: HashValue(4),
        key: 2,
    };
    
    let (index, value_ref) = vacant_entry.insert_sorted("unsorted value".to_string());
    
    assert_eq!(value_ref, &mut entries[index].value);
    assert_eq!(entries[index].value, "unsorted value".to_string());
}

#[test]
fn test_insert_sorted_at_end() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue(2), key: 2, value: "two".to_string() },
    ];
    
    let r_mut = RefMut {
        indices: &mut [],
        entries: &mut entries,
    };
    
    let vacant_entry = VacantEntry {
        map: r_mut,
        hash: HashValue(3),
        key: 3,
    };
    
    let (index, value_ref) = vacant_entry.insert_sorted("end value".to_string());
    
    assert_eq!(index, 2);
    assert_eq!(value_ref, &mut entries[index].value);
    assert_eq!(entries[index].value, "end value".to_string());
}

#[test]
fn test_insert_sorted_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    let mut entries: Vec<Bucket<i32, String>> = vec![];
    
    let r_mut = RefMut {
        indices: &mut [],
        entries: &mut entries,
    };
    
    let vacant_entry = VacantEntry {
        map: r_mut,
        hash: HashValue(1),
        key: 1,
    };
    
    let (index, value_ref) = vacant_entry.insert_sorted("first value".to_string());
    
    assert_eq!(index, 0);
    assert_eq!(value_ref, &mut entries[index].value);
    assert_eq!(entries[index].value, "first value".to_string());
}

