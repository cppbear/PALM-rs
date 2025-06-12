// Answer 0

#[test]
fn test_binary_search_by_key_empty() {
    let slice: Slice<i32> = Slice::new();
    let value = 5;
    let result = slice.binary_search_by_key(&value, |x| *x);
}

#[test]
fn test_binary_search_by_key_single_element_found() {
    let entries = vec![Bucket { hash: 0, key: 5, value: 10 }];
    let slice = Slice { entries: entries.as_slice() };
    let value = 5;
    let result = slice.binary_search_by_key(&value, |x| *x);
}

#[test]
fn test_binary_search_by_key_single_element_not_found() {
    let entries = vec![Bucket { hash: 0, key: 5, value: 10 }];
    let slice = Slice { entries: entries.as_slice() };
    let value = 3;
    let result = slice.binary_search_by_key(&value, |x| *x);
}

#[test]
fn test_binary_search_by_key_multiple_elements_found() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
        Bucket { hash: 0, key: 3, value: 30 },
    ];
    let slice = Slice { entries: entries.as_slice() };
    let value = 2;
    let result = slice.binary_search_by_key(&value, |x| *x);
}

#[test]
fn test_binary_search_by_key_multiple_elements_not_found() {
    let entries = vec![
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
        Bucket { hash: 0, key: 3, value: 30 },
    ];
    let slice = Slice { entries: entries.as_slice() };
    let value = 4;
    let result = slice.binary_search_by_key(&value, |x| *x);
}

#[test]
fn test_binary_search_by_key_max_elements_found() {
    let entries: Vec<Bucket<i32, i32>> = (0..u32::MAX).map(|x| Bucket { hash: 0, key: x as i32, value: x as i32 }).collect();
    let slice = Slice { entries: entries.as_slice() };
    let value = u32::MAX as i32;
    let result = slice.binary_search_by_key(&value, |x| *x);
}

#[test]
fn test_binary_search_by_key_max_elements_not_found() {
    let entries: Vec<Bucket<i32, i32>> = (0..u32::MAX).map(|x| Bucket { hash: 0, key: x as i32, value: x as i32 }).collect();
    let slice = Slice { entries: entries.as_slice() };
    let value = (u32::MAX + 1) as i32; // value outside the range
    let result = slice.binary_search_by_key(&value, |x| *x);
}

