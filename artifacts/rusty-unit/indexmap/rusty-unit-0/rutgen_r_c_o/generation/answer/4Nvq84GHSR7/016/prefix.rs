// Answer 0

#[test]
fn test_erase_indices_case_erased_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.extend(vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
    ]);
    map.indices.insert_unique(0, 0, |_| unreachable!());
    map.indices.insert_unique(1, 1, |_| unreachable!());
    map.indices.insert_unique(2, 2, |_| unreachable!());

    map.erase_indices(2, 3);
}

#[test]
fn test_erase_indices_case_start_plus_shifted_less_half_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.entries.extend(vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
        Bucket { hash: HashValue(4), key: 4, value: 4 },
    ]);
    map.indices.insert_unique(0, 0, |_| unreachable!());
    map.indices.insert_unique(1, 1, |_| unreachable!());
    map.indices.insert_unique(2, 2, |_| unreachable!());
    map.indices.insert_unique(3, 3, |_| unreachable!());
    map.indices.insert_unique(4, 4, |_| unreachable!());

    map.erase_indices(1, 3);
}

#[test]
fn test_erase_indices_case_erased_plus_shifted_less_half_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.entries.extend(vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
        Bucket { hash: HashValue(4), key: 4, value: 4 },
        Bucket { hash: HashValue(5), key: 5, value: 5 },
        Bucket { hash: HashValue(6), key: 6, value: 6 },
    ]);
    map.indices.insert_unique(0, 0, |_| unreachable!());
    map.indices.insert_unique(1, 1, |_| unreachable!());
    map.indices.insert_unique(2, 2, |_| unreachable!());
    map.indices.insert_unique(3, 3, |_| unreachable!());
    map.indices.insert_unique(4, 4, |_| unreachable!());
    map.indices.insert_unique(5, 5, |_| unreachable!());

    map.erase_indices(3, 6);
}

#[test]
fn test_erase_indices_case_shifted_adjustments() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.entries.extend(vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
        Bucket { hash: HashValue(4), key: 4, value: 4 },
        Bucket { hash: HashValue(5), key: 5, value: 5 },
        Bucket { hash: HashValue(6), key: 6, value: 6 },
        Bucket { hash: HashValue(7), key: 7, value: 7 },
    ]);
    map.indices.insert_unique(0, 0, |_| unreachable!());
    map.indices.insert_unique(1, 1, |_| unreachable!());
    map.indices.insert_unique(2, 2, |_| unreachable!());
    map.indices.insert_unique(3, 3, |_| unreachable!());
    map.indices.insert_unique(4, 4, |_| unreachable!());
    map.indices.insert_unique(5, 5, |_| unreachable!());
    map.indices.insert_unique(6, 6, |_| unreachable!());
    map.indices.insert_unique(7, 7, |_| unreachable!());

    map.erase_indices(2, 5);
}

#[test]
fn test_erase_indices_case_bounds_check() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.entries.extend(vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
        Bucket { hash: HashValue(4), key: 4, value: 4 },
        Bucket { hash: HashValue(5), key: 5, value: 5 },
        Bucket { hash: HashValue(6), key: 6, value: 6 },
        Bucket { hash: HashValue(7), key: 7, value: 7 },
        Bucket { hash: HashValue(8), key: 8, value: 8 },
        Bucket { hash: HashValue(9), key: 9, value: 9 },
    ]);
    map.indices.insert_unique(0, 0, |_| unreachable!());
    map.indices.insert_unique(1, 1, |_| unreachable!());
    map.indices.insert_unique(2, 2, |_| unreachable!());
    map.indices.insert_unique(3, 3, |_| unreachable!());
    map.indices.insert_unique(4, 4, |_| unreachable!());
    map.indices.insert_unique(5, 5, |_| unreachable!());
    map.indices.insert_unique(6, 6, |_| unreachable!());
    map.indices.insert_unique(7, 7, |_| unreachable!());
    map.indices.insert_unique(8, 8, |_| unreachable!());
    map.indices.insert_unique(9, 9, |_| unreachable!());

    map.erase_indices(0, 10);
}

