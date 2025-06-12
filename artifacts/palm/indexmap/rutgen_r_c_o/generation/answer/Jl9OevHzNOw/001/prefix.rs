// Answer 0

#[test]
fn test_binary_search_empty() {
    let slice: Box<Slice<i32>> = Box::new(Slice::new());
    let result = slice.binary_search(&1);
}

#[test]
fn test_binary_search_single_element_found() {
    let slice: Box<Slice<i32>> = Box::new(Slice {
        entries: [Bucket { hash: 0, key: 1, value: 10 }],
    });
    let result = slice.binary_search(&1);
}

#[test]
fn test_binary_search_single_element_not_found() {
    let slice: Box<Slice<i32>> = Box::new(Slice {
        entries: [Bucket { hash: 0, key: 1, value: 10 }],
    });
    let result = slice.binary_search(&2);
}

#[test]
fn test_binary_search_multiple_elements_found() {
    let slice: Box<Slice<i32>> = Box::new(Slice {
        entries: [
            Bucket { hash: 1, key: 1, value: 10 },
            Bucket { hash: 2, key: 3, value: 30 },
            Bucket { hash: 3, key: 5, value: 50 },
        ],
    });
    let result = slice.binary_search(&3);
}

#[test]
fn test_binary_search_multiple_elements_not_found() {
    let slice: Box<Slice<i32>> = Box::new(Slice {
        entries: [
            Bucket { hash: 1, key: 1, value: 10 },
            Bucket { hash: 2, key: 3, value: 30 },
            Bucket { hash: 3, key: 5, value: 50 },
        ],
    });
    let result = slice.binary_search(&4);
}

#[test]
fn test_binary_search_upper_bound() {
    let slice: Box<Slice<i32>> = Box::new(Slice {
        entries: [
            Bucket { hash: 1, key: 0, value: 0 },
            Bucket { hash: 2, key: 1_000_000, value: 100 },
        ],
    });
    let result = slice.binary_search(&1_000_000);
}

#[test]
fn test_binary_search_lower_bound() {
    let slice: Box<Slice<i32>> = Box::new(Slice {
        entries: [
            Bucket { hash: 1, key: 1, value: 10 },
            Bucket { hash: 2, key: 2, value: 20 },
        ],
    });
    let result = slice.binary_search(&0);
}

#[test]
fn test_binary_search_large_input() {
    let entries: Vec<Bucket<i32>> = (0..1_000_000).map(|x| Bucket { hash: x as u64, key: x, value: x }).collect();
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: entries.try_into().unwrap() });
    let result = slice.binary_search(&500_000);
}

