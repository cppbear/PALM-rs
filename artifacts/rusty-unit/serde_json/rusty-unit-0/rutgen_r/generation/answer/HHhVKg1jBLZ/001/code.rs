// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use indexmap::IndexMap;

    struct Map {
        #[cfg(not(feature = "preserve_order"))]
        map: BTreeMap<String, String>,
        #[cfg(feature = "preserve_order")]
        map: IndexMap<String, String>,
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
        #[cfg(not(feature = "preserve_order"))]
        assert_eq!(map.map.len(), 0);
        #[cfg(feature = "preserve_order")]
        assert_eq!(map.map.capacity(), 0);
    }

    #[test]
    fn test_with_capacity_nonzero() {
        let map = Map::with_capacity(10);
        #[cfg(not(feature = "preserve_order"))]
        assert_eq!(map.map.len(), 0);
        #[cfg(feature = "preserve_order")]
        assert_eq!(map.map.capacity(), 10);
    }

    #[test]
    fn test_with_capacity_large_value() {
        let map = Map::with_capacity(1_000_000);
        #[cfg(not(feature = "preserve_order"))]
        assert_eq!(map.map.len(), 0);
        #[cfg(feature = "preserve_order")]
        assert_eq!(map.map.capacity(), 1_000_000);
    }
}

