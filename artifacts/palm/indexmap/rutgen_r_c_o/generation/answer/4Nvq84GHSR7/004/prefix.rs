// Answer 0

#[test]
fn test_erase_indices_no_erased() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 20 });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.erase_indices(0, 0);
}

#[test]
fn test_erase_indices_start_shifted_less_than_half_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: 30 });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 4, value: 40 });
    index_map.indices.insert_unique(3, 0, |_| unreachable!());
    index_map.indices.insert_unique(4, 1, |_| unreachable!());
    index_map.erase_indices(0, 2);
}

#[test]
fn test_erase_indices_non_empty_erased() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    index_map.entries.push(Bucket { hash: HashValue(5), key: 5, value: 50 });
    index_map.entries.push(Bucket { hash: HashValue(6), key: 6, value: 60 });
    index_map.entries.push(Bucket { hash: HashValue(7), key: 7, value: 70 });
    index_map.indices.insert_unique(5, 0, |_| unreachable!());
    index_map.indices.insert_unique(6, 1, |_| unreachable!());
    index_map.indices.insert_unique(7, 2, |_| unreachable!());
    index_map.erase_indices(1, 3);
}

#[test]
fn test_erase_indices_shifted_full_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(20);
    for i in 0..5 {
        index_map.entries.push(Bucket { hash: HashValue(i as usize), key: i, value: i * 10 });
        index_map.indices.insert_unique(i as usize, i, |_| unreachable!());
    }
    index_map.erase_indices(0, 3);
}

