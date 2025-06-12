// Answer 0

#[test]
fn test_get_many_mut_unique_hashes() {
    let mut table: RawTable<u64> = RawTable::new_in(Global);
    let hashes = [0, 1, 2, 3];
    let values = [10, 20, 30, 40];
    for (hash, value) in hashes.iter().zip(values.iter()) {
        unsafe {
            table.insert(*hash, *value, |x| *x);
        }
    }
    let _result = table.get_many_mut(hashes, |i, &v| v == values[i]);
}

#[test]
fn test_get_many_mut_duplicate_hashes() {
    let mut table: RawTable<u64> = RawTable::new_in(Global);
    let hashes = [1, 1, 2, 3];
    let values = [10, 20, 30, 40];
    for (hash, value) in hashes.iter().zip(values.iter()) {
        unsafe {
            table.insert(*hash, *value, |x| *x);
        }
    }
    let _result = table.get_many_mut(hashes, |i, &v| v == values[i]);
}

#[test]
fn test_get_many_mut_non_existent_hashes() {
    let mut table: RawTable<u64> = RawTable::new_in(Global);
    let hashes = [4, 5, 6, 7];
    let _result = table.get_many_mut(hashes, |i, _| false);
}

