// Answer 0

#[test]
fn test_get_range_empty() {
    let slice = Slice::new();
    let _ = slice.get_range(0..0);
    let _ = slice.get_range(..0);
    let _ = slice.get_range(0..=0);
}

#[test]
fn test_get_range_one_element() {
    let entries = vec![Bucket { hash: 0, key: 1, value: 2 }];
    let slice = Box::new(Slice { entries });
    let _ = slice.get_range(0..1);
    let _ = slice.get_range(0..=0);
    let _ = slice.get_range(..1);
}

#[test]
fn test_get_range_multiple_elements() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 2 },
        Bucket { hash: 1, key: 3, value: 4 },
        Bucket { hash: 2, key: 5, value: 6 },
    ];
    let slice = Box::new(Slice { entries });
    let _ = slice.get_range(0..2);
    let _ = slice.get_range(1..=2);
    let _ = slice.get_range(..2);
    let _ = slice.get_range(2..=2);
}

#[test]
fn test_get_range_with_bounds() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 2 },
        Bucket { hash: 1, key: 3, value: 4 },
    ];
    let slice = Box::new(Slice { entries });
    let _ = slice.get_range((Bound::Included(0), Bound::Included(1)));
    let _ = slice.get_range((Bound::Excluded(0), Bound::Included(0)));
    let _ = slice.get_range((Bound::Unbounded, Bound::Included(1)));
    let _ = slice.get_range((Bound::Included(1), Bound::Unbounded));
}

#[test]
fn test_get_range_out_of_bounds() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 2 },
        Bucket { hash: 1, key: 3, value: 4 },
    ];
    let slice = Box::new(Slice { entries });
    let _ = slice.get_range(3..5);
    let _ = slice.get_range(2..=3);
    let _ = slice.get_range(..2);
}

#[test]
fn test_get_range_large_indices() {
    let entries: Vec<Bucket<usize, usize>> = (0..100).map(|i| Bucket { hash: i, key: i, value: i }).collect();
    let slice = Box::new(Slice { entries });
    let _ = slice.get_range(0..usize::MAX);
    let _ = slice.get_range(0..10);
    let _ = slice.get_range(90..usize::MAX);
}

