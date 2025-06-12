// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    struct IndexSet {
        map: IndexMap<usize, ()>,
    }

    impl IndexSet {
        pub fn with_capacity(n: usize) -> Self {
            IndexSet {
                map: IndexMap::with_capacity(n),
            }
        }
    }

    #[test]
    fn test_with_capacity_zero() {
        let set = IndexSet::with_capacity(0);
        assert!(set.map.is_empty());
    }

    #[test]
    fn test_with_capacity_small() {
        let set = IndexSet::with_capacity(1);
        assert!(set.map.is_empty());
        set.map.insert(0, ());
        assert_eq!(set.map.len(), 1);
    }

    #[test]
    fn test_with_capacity_large() {
        let n = 1000;
        let set = IndexSet::with_capacity(n);
        assert!(set.map.is_empty());
        for i in 0..n {
            set.map.insert(i, ());
        }
        assert_eq!(set.map.len(), n);
    }

    #[test]
    #[should_panic]
    fn test_with_capacity_negative() {
        // This is a forced panic due to the violation of type constraints,
        // Rust does not allow negative usize values, this is illustrative only.
        let _set = IndexSet::with_capacity(usize::max_value());
    }
}

