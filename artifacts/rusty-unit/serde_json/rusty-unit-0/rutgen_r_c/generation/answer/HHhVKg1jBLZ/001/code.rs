// Answer 0

#[test]
fn test_with_capacity_btree_map() {
    #[cfg(not(feature = "preserve_order"))]
    {
        let capacity = 10;
        let map = Map::with_capacity(capacity);
        assert_eq!(map.len(), 0);
    }
}

#[test]
fn test_with_capacity_index_map() {
    #[cfg(feature = "preserve_order")] 
    {
        let capacity = 10;
        let map = Map::with_capacity(capacity);
        assert_eq!(map.len(), 0);
    }
}

#[test]
fn test_with_capacity_zero() {
    let capacity = 0;
    let map = Map::with_capacity(capacity);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_with_capacity_large_number() {
    let capacity = usize::MAX;
    let map = Map::with_capacity(capacity);
    assert_eq!(map.len(), 0);
}

