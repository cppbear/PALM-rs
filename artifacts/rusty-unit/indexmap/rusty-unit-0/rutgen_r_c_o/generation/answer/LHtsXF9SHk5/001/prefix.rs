// Answer 0

#[test]
fn test_new_empty_entries() {
    let entries: &[Bucket<i32, &str>] = &[];
    let values = Values::new(entries);
}

#[test]
fn test_new_single_entry() {
    let entries: &[Bucket<i32, &str>] = &[Bucket { hash: 0, key: 1, value: "one" }];
    let values = Values::new(entries);
}

#[test]
fn test_new_multiple_entries() {
    let entries: &[Bucket<i32, &str>] = &[
        Bucket { hash: 0, key: 1, value: "one" },
        Bucket { hash: 1, key: 2, value: "two" },
        Bucket { hash: 2, key: 3, value: "three" },
    ];
    let values = Values::new(entries);
}

#[test]
fn test_new_full_capacity_entries() {
    let max_size = 10;
    let mut entries: Vec<Bucket<i32, &str>> = Vec::with_capacity(max_size);
    for i in 0..max_size {
        entries.push(Bucket { hash: i, key: i as i32, value: "value" });
    }
    let values = Values::new(&entries);
}

