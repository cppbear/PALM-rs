// Answer 0

#[test]
fn test_shift_remove_finish_valid_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, String>> = vec![
        Bucket { hash: HashValue::default(), key: 0, value: String::from("zero") },
        Bucket { hash: HashValue::default(), key: 1, value: String::from("one") },
        Bucket { hash: HashValue::default(), key: 2, value: String::from("two") },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let (key, value) = ref_mut.shift_remove_finish(1);
}

#[test]
fn test_shift_remove_finish_first_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, String>> = vec![
        Bucket { hash: HashValue::default(), key: 0, value: String::from("first") },
        Bucket { hash: HashValue::default(), key: 1, value: String::from("second") },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let (key, value) = ref_mut.shift_remove_finish(0);
}

#[test]
fn test_shift_remove_finish_last_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, String>> = vec![
        Bucket { hash: HashValue::default(), key: 0, value: String::from("last") },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let (key, value) = ref_mut.shift_remove_finish(0);
}

#[test]
#[should_panic]
fn test_shift_remove_finish_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, String>> = vec![
        Bucket { hash: HashValue::default(), key: 0, value: String::from("test") },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let (key, value) = ref_mut.shift_remove_finish(1);
}

