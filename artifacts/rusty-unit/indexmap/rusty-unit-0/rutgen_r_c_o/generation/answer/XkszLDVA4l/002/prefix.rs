// Answer 0

#[test]
fn test_split_off_out_of_bounds_negative() {
    let mut map = IndexMapCore::<usize, String>::new();
    let index = 1; // at is greater than len (len is 0)
    map.split_off(index);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds_exceed_capacity() {
    let mut map = IndexMapCore::<usize, String>::with_capacity(IndexMapCore::<usize, String>::MAX_ENTRIES_CAPACITY);
    let index = IndexMapCore::<usize, String>::MAX_ENTRIES_CAPACITY + 1; // at exceeds the MAX_ENTRIES_CAPACITY
    map.split_off(index);
}

#[test]
fn test_split_off_valid() {
    let mut map = IndexMapCore::<usize, String>::new();
    map.entries.push(Bucket { hash: HashValue::from(0), key: 1, value: "one".to_string() });
    map.entries.push(Bucket { hash: HashValue::from(1), key: 2, value: "two".to_string() });

    let index = 1; // splitting at a valid index
    let result = map.split_off(index);
}

#[test]
fn test_split_off_zero() {
    let mut map = IndexMapCore::<usize, String>::new();
    let index = 0; // splitting at zero should also be valid
    let result = map.split_off(index);
}

