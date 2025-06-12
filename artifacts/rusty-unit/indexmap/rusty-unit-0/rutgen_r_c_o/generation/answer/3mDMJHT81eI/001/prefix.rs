// Answer 0

#[test]
fn test_values_mut_empty() {
    let mut slice = Box::new(Slice::<i32, i32>::new_mut());
    let _ = slice.values_mut();
}

#[test]
fn test_values_mut_single() {
    let mut entries = [Bucket { hash: 0, key: 1, value: 10 }];
    let mut slice = Box::new(Slice { entries });
    let _ = slice.values_mut();
}

#[test]
fn test_values_mut_multiple() {
    let mut entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];
    let mut slice = Box::new(Slice { entries });
    let _ = slice.values_mut();
}

#[test]
fn test_values_mut_large() {
    let mut entries: Vec<Bucket<i32, i32>> = (0..1000)
        .map(|i| Bucket { hash: i, key: i, value: i * 10 })
        .collect();
    let mut slice = Box::new(Slice { entries: entries.try_into().unwrap() });
    let _ = slice.values_mut();
}

