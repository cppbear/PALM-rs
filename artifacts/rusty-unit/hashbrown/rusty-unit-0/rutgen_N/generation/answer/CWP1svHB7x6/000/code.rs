// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    struct TestSet {
        iter: Vec<(i32, i32)>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { iter: vec![] }
        }

        fn push(&mut self, key: i32, value: i32) {
            self.iter.push((key, value));
        }
    }

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let entries_iter = self.iter.iter().map(|(k, _)| k);
            f.debug_list().entries(entries_iter).finish()
        }
    }

    #[test]
    fn test_empty_set_debug_format() {
        let set = TestSet::new();
        let result = format!("{:?}", set);
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_non_empty_set_debug_format() {
        let mut set = TestSet::new();
        set.push(1, 10);
        set.push(2, 20);
        let result = format!("{:?}", set);
        assert_eq!(result, "[1, 2]");
    }

    #[test]
    fn test_single_element_set_debug_format() {
        let mut set = TestSet::new();
        set.push(42, 100);
        let result = format!("{:?}", set);
        assert_eq!(result, "[42]");
    }
}

