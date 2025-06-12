// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct MyMap {
        core: std::collections::HashMap<u32, u32>
    }

    impl MyMap {
        fn new() -> Self {
            MyMap {
                core: std::collections::HashMap::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.core.reserve(additional);
        }
    }

    #[test]
    fn test_reserve_with_positive_additional() {
        let mut my_map = MyMap::new();
        let initial_capacity = my_map.core.capacity();
        my_map.reserve(10);
        assert!(my_map.core.capacity() >= initial_capacity + 10);
    }

    #[test]
    fn test_reserve_with_zero_additional() {
        let mut my_map = MyMap::new();
        let initial_capacity = my_map.core.capacity();
        my_map.reserve(0);
        assert_eq!(my_map.core.capacity(), initial_capacity);
    }

    #[test]
    fn test_reserve_increases_capacity() {
        let mut my_map = MyMap::new();
        for i in 0..10 {
            my_map.core.insert(i, i * 2);
        }
        let initial_capacity = my_map.core.capacity();
        my_map.reserve(5);
        assert!(my_map.core.capacity() > initial_capacity);
    }
}

