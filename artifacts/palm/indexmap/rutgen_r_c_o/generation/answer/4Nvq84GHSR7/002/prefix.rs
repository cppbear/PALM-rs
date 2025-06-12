// Answer 0

#[test]
fn test_erase_indices_non_empty() {
    let mut index_map = IndexMapCore::new();

    // Prepare entries with hash values and keys/values
    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "a" });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "b" });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "c" });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());

    // Erase indices from start = 1 to end = 2
    index_map.erase_indices(1, 2);
}

#[test]
fn test_erase_indices_remain_in_order() {
    let mut index_map = IndexMapCore::new();

    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "a" });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "b" });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "c" });
    index_map.entries.push(Bucket { hash: HashValue(4), key: 4, value: "d" });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());
    index_map.indices.insert_unique(4, 3, |_| unreachable!());

    // Erase indices from start = 1 to end = 3
    index_map.erase_indices(1, 3);
}

#[test]
fn test_erase_indices_edge_case_beginning() {
    let mut index_map = IndexMapCore::new();

    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "a" });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "b" });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());

    // Erase indices from start = 0 to end = 2
    index_map.erase_indices(0, 2);
}

#[test]
fn test_erase_indices_edge_case_end() {
    let mut index_map = IndexMapCore::new();

    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "a" });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "b" });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "c" });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());

    // Erase indices from start = 1 to end = 3
    index_map.erase_indices(1, 3);
}

#[test]
fn test_erase_indices_no_shift() {
    let mut index_map = IndexMapCore::new();

    index_map.entries.push(Bucket { hash: HashValue(1), key: 1, value: "a" });
    index_map.entries.push(Bucket { hash: HashValue(2), key: 2, value: "b" });
    index_map.entries.push(Bucket { hash: HashValue(3), key: 3, value: "c" });
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());

    // Erase indices from start = 0 to end = 1
    index_map.erase_indices(0, 1);
}

