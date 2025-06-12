// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashSet;

    #[test]
    fn test_is_superset_with_empty_set() {
        let empty_set: HashSet<i32> = HashSet::new();
        let sub: HashSet<i32> = HashSet::new();
        assert!(empty_set.is_superset(&sub));
    }

    #[test]
    fn test_is_superset_with_non_empty_set() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        
        let sub: HashSet<i32> = vec![1].into_iter().collect();
        assert!(set.is_superset(&sub));
        
        let another_sub: HashSet<i32> = vec![1, 2].into_iter().collect();
        assert!(set.is_superset(&another_sub));

        let not_a_superset: HashSet<i32> = vec![3].into_iter().collect();
        assert!(!set.is_superset(&not_a_superset));
    }

    #[test]
    fn test_is_superset_with_equal_sets() {
        let set: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let another_set: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        assert!(set.is_superset(&another_set));
    }

    #[test]
    fn test_is_superset_with_different_elements() {
        let set: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let sub: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
        assert!(!set.is_superset(&sub));
    }
}

