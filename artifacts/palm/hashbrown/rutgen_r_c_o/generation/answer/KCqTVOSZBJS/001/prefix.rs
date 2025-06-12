// Answer 0

#[test]
fn test_fmt_with_empty_table() {
    let alloc = Global;
    let table: HashTable<i32, Global> = HashTable::new_in(alloc);
    let _ = format!("{:?}", table);
}

#[test]
fn test_fmt_with_single_element() {
    let alloc = Global;
    let mut table: HashTable<i32, Global> = HashTable::with_capacity_in(1, alloc);
    let hasher = |x: &i32| *x as u64;
    let _ = table.insert_unique(1, 42, hasher);
    let _ = format!("{:?}", table);
}

#[test]
fn test_fmt_with_multiple_elements() {
    let alloc = Global;
    let mut table: HashTable<i32, Global> = HashTable::with_capacity_in(10, alloc);
    let hasher = |x: &i32| *x as u64;
    for i in 1..=5 {
        let _ = table.insert_unique(i as u64, i * 10, hasher);
    }
    let _ = format!("{:?}", table);
}

#[test]
fn test_fmt_with_full_capacity() {
    let alloc = Global;
    let mut table: HashTable<i32, Global> = HashTable::with_capacity_in(1000, alloc);
    let hasher = |x: &i32| *x as u64;
    for i in 1..=1000 {
        let _ = table.insert_unique(i as u64, i, hasher);
    }
    let _ = format!("{:?}", table);
}

#[test]
fn test_fmt_with_panic_condition() {
    let alloc = Global;
    let mut table: HashTable<String, Global> = HashTable::with_capacity_in(2, alloc);
    let hasher = |x: &String| x.len() as u64;
    let _ = table.insert_unique(hasher(&"test".to_string()), "test".to_string(), hasher);
    let _ = table.insert_unique(hasher(&"example".to_string()), "example".to_string(), hasher);
    // Attempt to insert a duplicate to trigger a panic condition
    let _ = core::panic::catch_unwind(|| {
        let _ = table.insert_unique(hasher(&"test".to_string()), "test".to_string(), hasher);
    });
}

