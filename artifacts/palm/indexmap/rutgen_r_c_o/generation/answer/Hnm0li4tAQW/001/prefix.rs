// Answer 0

#[test]
fn test_iter_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice::new());
    let iter = slice.iter();
}

#[test]
fn test_iter_single_element_slice() {
    let mut entries = vec![Bucket { hash: 0, key: 1, value: 10 }];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: entries.try_into().unwrap() });
    let iter = slice.iter();
}

#[test]
fn test_iter_multiple_elements_slice() {
    let mut entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: entries.try_into().unwrap() });
    let iter = slice.iter();
}

#[test]
fn test_iter_large_slice() {
    let mut entries: Vec<Bucket<i32, i32>> = (0..1000)
        .map(|i| Bucket { hash: i, key: i, value: i * 10 })
        .collect();
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: entries.try_into().unwrap() });
    let iter = slice.iter();
}

