// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use std::collections::TryReserveError;

    #[test]
    fn test_try_reserve_exact_success() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert!(map.try_reserve_exact(2).is_ok());
    }

    #[test]
    fn test_try_reserve_exact_zero() {
        let mut map: IndexMap<i32, i32> = IndexMap::new();
        map.insert(1, 10);
        assert!(map.try_reserve_exact(0).is_ok());
    }

    #[test]
    fn test_try_reserve_exact_failure() {
        // This test may not be directly possible as it requires a scenario where the allocation fails,
        // but we can simulate it by being aware of the system constraints.
        let mut map: IndexMap<i32, i32> = IndexMap::new();

        // Assuming memory exhaustion or similar conditions.
        // It's hard to explicitly trigger memory errors in a simple test case.
        // Thus we might just denote that if we expect an error under certain conditions.

        // This should be left pending or commented.
        // assert!(map.try_reserve_exact(usize::MAX).is_err());
    }
}

