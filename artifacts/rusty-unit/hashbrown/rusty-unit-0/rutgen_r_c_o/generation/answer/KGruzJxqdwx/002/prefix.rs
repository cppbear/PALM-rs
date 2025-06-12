// Answer 0

#[test]
fn test_entry_occupied_minimum() {
    let mut table = HashMap::with_capacity(1);
    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket::new(),
        table: &mut table,
    };
    let entry = Entry::Occupied(occupied_entry);
    let _ = fmt(&entry);
}

#[test]
fn test_entry_occupied_maximum() {
    let mut table = HashMap::with_capacity(1);
    let occupied_entry = OccupiedEntry {
        hash: u64::MAX,
        elem: Bucket::new(),
        table: &mut table,
    };
    let entry = Entry::Occupied(occupied_entry);
    let _ = fmt(&entry);
}

#[test]
fn test_entry_occupied_random() {
    let mut table = HashMap::with_capacity(1);
    let occupied_entry = OccupiedEntry {
        hash: 12345,
        elem: Bucket::new(),
        table: &mut table,
    };
    let entry = Entry::Occupied(occupied_entry);
    let _ = fmt(&entry);
}

#[test]
fn test_entry_occupied_non_empty_bucket() {
    let mut table = HashMap::with_capacity(1);
    let bucket = Bucket::from_vec(vec![(1, "value")]);
    let occupied_entry = OccupiedEntry {
        hash: 67890,
        elem: bucket,
        table: &mut table,
    };
    let entry = Entry::Occupied(occupied_entry);
    let _ = fmt(&entry);
}

#[test]
fn test_entry_occupied_large_bucket() {
    let mut table = HashMap::with_capacity(1);
    let bucket = Bucket::from_vec((0..1000).map(|i| (i, i.to_string())).collect());
    let occupied_entry = OccupiedEntry {
        hash: 54321,
        elem: bucket,
        table: &mut table,
    };
    let entry = Entry::Occupied(occupied_entry);
    let _ = fmt(&entry);
}

