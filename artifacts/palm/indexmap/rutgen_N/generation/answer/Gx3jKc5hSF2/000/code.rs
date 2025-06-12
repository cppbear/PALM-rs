// Answer 0

#[test]
fn test_reverse_empty() {
    let mut map = IndexMap::new();
    map.reverse();
    assert_eq!(map.entries.len(), 0);
}

#[test]
fn test_reverse_single_entry() {
    let mut map = IndexMap::new();
    map.entries.push("a");
    map.indices.push(0);
    map.reverse();
    assert_eq!(map.entries, vec!["a"]);
    assert_eq!(map.indices, vec![0]);
}

#[test]
fn test_reverse_multiple_entries() {
    let mut map = IndexMap::new();
    map.entries.push("a");
    map.entries.push("b");
    map.entries.push("c");
    map.indices.push(0);
    map.indices.push(1);
    map.indices.push(2);
    map.reverse();
    assert_eq!(map.entries, vec!["c", "b", "a"]);
    assert_eq!(map.indices, vec![2, 1, 0]);
}

#[test]
fn test_reverse_with_duplicates() {
    let mut map = IndexMap::new();
    map.entries.push("a");
    map.entries.push("b");
    map.entries.push("a");
    map.indices.push(0);
    map.indices.push(1);
    map.indices.push(2);
    map.reverse();
    assert_eq!(map.entries, vec!["a", "b", "a"]);
    assert_eq!(map.indices, vec![2, 1, 0]);
}

#[test]
fn test_reverse_indices() {
    let mut map = IndexMap::new();
    map.entries.push("first");
    map.entries.push("second");
    map.entries.push("third");
    map.indices.push(0);
    map.indices.push(1);
    map.indices.push(2);
    map.reverse();
    assert_eq!(map.indices, vec![2, 1, 0]);
}

