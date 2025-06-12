// Answer 0

#[test]
fn test_get_many_mut_single_entry() {
    let mut table = RawTable::new_in(Global);
    let hash = 0;
    let value = 42;
    unsafe {
        table.insert(hash, value, |&x| x);
    }
    let hashes = [hash];
    table.get_many_mut(hashes, |_, &k| k == value);
}

#[test]
fn test_get_many_mut_multiple_entries() {
    let mut table = RawTable::new_in(Global);
    let value1 = 42;
    let value2 = 43;
    let value3 = 44;
    let hash1 = 0;
    let hash2 = 1;
    let hash3 = 2;
    
    unsafe {
        table.insert(hash1, value1, |&x| x);
        table.insert(hash2, value2, |&x| x);
        table.insert(hash3, value3, |&x| x);
    }
    
    let hashes = [hash1, hash2];
    table.get_many_mut(hashes, |_, &k| k == value1 || k == value2);
}

#[test]
fn test_get_many_mut_edge_case_no_entries() {
    let mut table = RawTable::new_in(Global);
    let hashes = [0];
    let result: [Option<&mut _>; 1] = table.get_many_mut(hashes, |_, _| false);
}

#[test]
#[should_panic(expected = "duplicate keys found")]
fn test_get_many_mut_duplicate_keys() {
    let mut table = RawTable::new_in(Global);
    let value = 42;
    let hash = 0;
    
    unsafe {
        table.insert(hash, value, |&x| x);
    }
    
    let hashes = [hash, hash]; // Duplicate hash
    table.get_many_mut(hashes, |_, &k| k == value);
}

#[test]
fn test_get_many_mut_non_existent_hash() {
    let mut table = RawTable::new_in(Global);
    let value = 42;
    let hash = 0;
    
    unsafe {
        table.insert(hash, value, |&x| x);
    }
    
    let hashes = [1]; // Non-existent hash
    let result: [Option<&mut _>; 1] = table.get_many_mut(hashes, |_, &k| k == value);
}

#[test]
fn test_get_many_mut_unique_hashes() {
    let mut table = RawTable::new_in(Global);
    let value1 = 42;
    let value2 = 43;
    let hash1 = 0;
    let hash2 = 1;
    
    unsafe {
        table.insert(hash1, value1, |&x| x);
        table.insert(hash2, value2, |&x| x);
    }
    
    let hashes = [hash1, hash2];
    table.get_many_mut(hashes, |_, &k| k == value1 || k == value2);
}

#[test]
fn test_get_many_mut_with_max_capacity() {
    let mut table = RawTable::with_capacity_in(10, Global);
    for i in 0..10 {
        unsafe {
            table.insert(i as u64, i * 10, |&x| x);
        }
    }
    
    let hashes: [u64; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    table.get_many_mut(hashes, |_, &k| k % 10 == 0);
}

