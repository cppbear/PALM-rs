// Answer 0

#[test]
fn test_is_empty_on_empty_slice() {
    let slice: &Slice<i32> = Slice::new();
    slice.is_empty();
}

#[test]
fn test_is_empty_on_non_empty_slice() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
    ];
    let slice = Slice { entries };
    slice.is_empty();
}

#[test]
fn test_is_empty_on_another_non_empty_slice() {
    let entries = vec![
        Bucket { hash: 1, key: 2, value: 20 },
        Bucket { hash: 2, key: 3, value: 30 },
    ];
    let slice = Slice { entries };
    slice.is_empty();
}

#[test]
fn test_is_empty_with_one_element() {
    let entries = vec![
        Bucket { hash: 3, key: 4, value: 40 },
    ];
    let slice = Slice { entries };
    slice.is_empty();
}

#[test]
fn test_is_empty_on_large_slice() {
    let entries: Vec<Bucket<i32, i32>> = (0..100).map(|i| Bucket { hash: i, key: i, value: i }).collect();
    let slice = Slice { entries };
    slice.is_empty();
}

