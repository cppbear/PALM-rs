// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashSet;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_hashset_with_capacity_zero() {
        let hasher = BuildHasherDefault::<RandomState>::default();
        let set: HashSet<i32, BuildHasherDefault<RandomState>> =
            HashSet::with_capacity_and_hasher_in(0, hasher, ());
        assert!(set.is_empty());
    }

    #[test]
    fn test_hashset_with_non_zero_capacity() {
        let hasher = BuildHasherDefault::<RandomState>::default();
        let mut set: HashSet<i32, BuildHasherDefault<RandomState>> =
            HashSet::with_capacity_and_hasher_in(10, hasher, ());
        set.insert(1);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));
    }

    #[test]
    #[should_panic(expected = "attempted to create a zero capacity HashSet with a non-zero hasher")]
    fn test_hashset_with_no_capacity_and_non_zero_hasher() {
        let hasher = BuildHasherDefault::<RandomState>::default();
        HashSet::with_capacity_and_hasher_in(0, hasher, 1);
    }
}

