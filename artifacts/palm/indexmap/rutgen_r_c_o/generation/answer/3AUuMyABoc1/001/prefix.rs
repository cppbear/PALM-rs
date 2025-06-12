// Answer 0

#[test]
fn test_sort_unstable_empty() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.sort_unstable();
}

#[test]
fn test_sort_unstable_single_element() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.map.insert(42, ());
    index_set.sort_unstable();
}

#[test]
fn test_sort_unstable_multiple_elements() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.map.insert(20, ());
    index_set.map.insert(10, ());
    index_set.map.insert(30, ());
    index_set.sort_unstable();
}

#[test]
fn test_sort_unstable_reversed_order() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.map.insert(30, ());
    index_set.map.insert(20, ());
    index_set.map.insert(10, ());
    index_set.sort_unstable();
}

#[test]
fn test_sort_unstable_large_set() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    for i in (0..10_000).rev() {
        index_set.map.insert(i, ());
    }
    index_set.sort_unstable();
}

#[test]
fn test_sort_unstable_with_duplicates() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    index_set.map.insert(10, ());
    index_set.map.insert(10, ());
    index_set.map.insert(20, ());
    index_set.sort_unstable();
}

#[test]
fn test_sort_unstable_large_range() {
    let mut index_set: IndexSet<u32, RandomState> = IndexSet { map: IndexMap::new() };
    for i in 0..1_000_000 {
        index_set.map.insert(i, ());
    }
    index_set.sort_unstable();
}

