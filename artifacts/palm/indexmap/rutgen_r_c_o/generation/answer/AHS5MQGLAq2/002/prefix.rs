// Answer 0

#[test]
fn test_split_last_empty() {
    let slice: Box<Slice<u32, u32>> = Box::new(Slice { entries: [] });
    let result = slice.split_last();
}

#[test]
fn test_split_last_single_entry() {
    let entries = [Bucket { hash: 0, key: 1u32, value: 10u32 }];
    let slice: Box<Slice<u32, u32>> = Box::new(Slice { entries });
    let result = slice.split_last();
}

#[test]
fn test_split_last_two_entries() {
    let entries = [
        Bucket { hash: 1, key: 1u32, value: 10u32 },
        Bucket { hash: 2, key: 2u32, value: 20u32 },
    ];
    let slice: Box<Slice<u32, u32>> = Box::new(Slice { entries });
    let result = slice.split_last();
}

