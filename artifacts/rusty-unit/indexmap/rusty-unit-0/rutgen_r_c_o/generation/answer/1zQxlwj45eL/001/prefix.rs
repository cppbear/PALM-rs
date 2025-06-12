// Answer 0

#[test]
fn test_shift_insert_valid_index() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, usize>::new();
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(1),
        key: 42,
    };
    let index = 0;
    let value = 100;
    vacant_entry.shift_insert(index, value);
}

#[test]
fn test_shift_insert_end_index() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, usize>::new();
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(2),
        key: 43,
    };
    let index = entries.len();
    let value = 200;
    vacant_entry.shift_insert(index, value);
}

#[test]
#[should_panic]
fn test_shift_insert_panic_out_of_bounds() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, usize>::new();
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(3),
        key: 44,
    };
    let index = 2; // Out of bounds
    let value = 300;
    vacant_entry.shift_insert(index, value);
}

#[test]
fn test_shift_insert_middle_index() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, usize>::new();
    entries.push(Bucket { hash: HashValue(1), key: 0, value: 10 });
    entries.push(Bucket { hash: HashValue(2), key: 1, value: 20 });
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(4),
        key: 45,
    };
    let index = 1; // Middle index
    let value = 250;
    vacant_entry.shift_insert(index, value);
}

#[test]
fn test_shift_insert_with_capacity() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, usize>::with_capacity(2);
    entries.push(Bucket { hash: HashValue(5), key: 2, value: 30 });
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry {
        map: ref_mut,
        hash: HashValue(6),
        key: 46,
    };
    let index = 1; // Valid index
    let value = 400;
    vacant_entry.shift_insert(index, value);
}

