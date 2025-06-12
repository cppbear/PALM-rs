// Answer 0

#[test]
fn test_into_values_new_empty() {
    let entries: Vec<Bucket<i32, i32>> = vec![];
    let into_values = IntoValues::new(entries);
    assert!(into_values.iter.len() == 0);
}

#[test]
fn test_into_values_new_single_entry() {
    let entries = vec![Bucket { hash: 0, key: 1, value: 100 }];
    let into_values = IntoValues::new(entries);
    let mut iter = into_values.iter;
    assert_eq!(iter.next().unwrap(), Bucket { hash: 0, key: 1, value: 100 });
    assert!(iter.next().is_none());
}

#[test]
fn test_into_values_new_multiple_entries() {
    let entries = vec![
        Bucket { hash: 1, key: 2, value: 200 },
        Bucket { hash: 2, key: 3, value: 300 },
        Bucket { hash: 3, key: 4, value: 400 },
    ];
    let into_values = IntoValues::new(entries);
    let mut iter = into_values.iter;
    
    assert_eq!(iter.next().unwrap(), Bucket { hash: 1, key: 2, value: 200 });
    assert_eq!(iter.next().unwrap(), Bucket { hash: 2, key: 3, value: 300 });
    assert_eq!(iter.next().unwrap(), Bucket { hash: 3, key: 4, value: 400 });
    assert!(iter.next().is_none());
}

