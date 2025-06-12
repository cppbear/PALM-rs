// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use indexmap::IndexMap;

    struct Map {
        #[cfg(not(feature = "preserve_order"))]
        map: BTreeMap<u32, u32>,
        #[cfg(feature = "preserve_order")]
        map: IndexMap<u32, u32>,
    }

    impl Map {
        pub fn with_capacity(capacity: usize) -> Self {
            Map {
                #[cfg(not(feature = "preserve_order"))]
                map: {
                    let _ = capacity;
                    BTreeMap::new()
                },
                #[cfg(feature = "preserve_order")]
                map: IndexMap::with_capacity(capacity),
            }
        }
    }

    #[test]
    fn test_with_capacity_zero() {
        let map = Map::with_capacity(0);
        #[cfg(feature = "preserve_order")]
        assert!(map.map.is_empty());
    }
    
    #[test]
    fn test_with_capacity_positive() {
        let capacity = 10;
        let map = Map::with_capacity(capacity);
        #[cfg(feature = "preserve_order")]
        assert!(map.map.capacity() >= capacity);
    }
    
    #[test]
    #[should_panic]
    fn test_with_capacity_negative() {
        let _ = Map::with_capacity(usize::MAX);
    }
    
    #[test]
    fn test_with_capacity_large_number() {
        let capacity = 1_000_000;
        let map = Map::with_capacity(capacity);
        #[cfg(feature = "preserve_order")]
        assert!(map.map.capacity() >= capacity);
    }
}

