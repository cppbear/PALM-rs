// Answer 0

#[test]
fn test_get_range_empty() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [] });
    assert_eq!(slice.get_range(0..1), None);
}

#[test]
fn test_get_range_out_of_bounds() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [] });
    assert_eq!(slice.get_range(1..2), None);
}

#[test]
fn test_get_range_full() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: entries });
    let result = slice.get_range(0..2).unwrap();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_get_range_partial() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: entries });
    let result = slice.get_range(0..1).unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_get_range_excluded() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: entries });
    let result = slice.get_range(0..=0).unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn test_get_range_with_bounds() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: entries });
    let result = slice.get_range(1..=1).unwrap();
    assert_eq!(result.len(), 1);
}

