// Answer 0

#[test]
fn test_into_entries_empty() {
    let slice: Box<Slice<u32, u32>> = Box::new(Slice::new());
    let _ = slice.into_entries();
}

#[test]
fn test_into_entries_single() {
    let mut entries = vec![Bucket { hash: 0, key: 1u32, value: 10u32 }];
    let slice: Box<Slice<u32, u32>> = Box::new(Slice::from_slice(&entries));
    let _ = slice.into_entries();
}

#[test]
fn test_into_entries_multiple() {
    let mut entries = vec![
        Bucket { hash: 0, key: 1u32, value: 10u32 },
        Bucket { hash: 1, key: 2u32, value: 20u32 },
        Bucket { hash: 2, key: 3u32, value: 30u32 },
    ];
    let slice: Box<Slice<u32, u32>> = Box::new(Slice::from_slice(&entries));
    let _ = slice.into_entries();
}

#[test]
fn test_into_entries_large() {
    let mut entries = (0..1000).map(|i| Bucket { hash: i, key: i, value: i * 10 }).collect::<Vec<_>>();
    let slice: Box<Slice<u32, u32>> = Box::new(Slice::from_slice(&entries));
    let _ = slice.into_entries();
}

