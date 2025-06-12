// Answer 0

#[test]
fn test_shift_remove_index_existing_entry() {
    let mut indices = Indices::new();
    let mut entries = vec![
        Bucket {
            hash: HashValue(1),
            key: 1,
            value: "a",
        },
        Bucket {
            hash: HashValue(2),
            key: 2,
            value: "b",
        },
        Bucket {
            hash: HashValue(3),
            key: 3,
            value: "c",
        },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let removed = ref_mut.shift_remove_index(1);
    
    assert_eq!(removed, Some((2, "b")));
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0], Bucket { hash: HashValue(1), key: 1, value: "a" });
    assert_eq!(entries[1], Bucket { hash: HashValue(3), key: 3, value: "c" });
}

#[test]
fn test_shift_remove_index_non_existent_entry() {
    let mut indices = Indices::new();
    let mut entries = vec![
        Bucket {
            hash: HashValue(1),
            key: 1,
            value: "a",
        },
        Bucket {
            hash: HashValue(2),
            key: 2,
            value: "b",
        },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let removed = ref_mut.shift_remove_index(2);
    
    assert_eq!(removed, None);
    assert_eq!(entries.len(), 2);
}

