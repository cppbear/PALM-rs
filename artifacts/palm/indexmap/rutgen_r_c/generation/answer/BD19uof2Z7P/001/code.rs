// Answer 0

#[test]
fn test_get_range_empty_slice() {
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries: [] });
    let result = slice.get_range(0..1);
    assert_eq!(result, None);
}

#[test]
fn test_get_range_out_of_bounds() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.get_range(2..4);
    assert_eq!(result, None);
}

#[test]
fn test_get_range_inclusive_bounds() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
        Bucket { hash: 0, key: 3, value: 30 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.get_range(0..=1);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_get_range_exclusive_bounds() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
        Bucket { hash: 0, key: 3, value: 30 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.get_range(1..3);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_get_range_with_unbounded_start() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.get_range(..=0);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn test_get_range_with_unbounded_end() {
    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
    ];
    let slice: Box<Slice<i32, i32>> = Box::new(Slice { entries });
    let result = slice.get_range(0..);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

