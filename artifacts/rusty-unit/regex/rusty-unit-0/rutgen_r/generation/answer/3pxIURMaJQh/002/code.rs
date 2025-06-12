// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDFA {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TestDFA {
        fn new(table: Vec<StatePtr>, num_byte_classes: usize) -> Self {
            Self { table, num_byte_classes }
        }
    }

    #[test]
    fn test_next_unchecked_valid_case() {
        let dfa = TestDFA::new(vec![1, 2, 3], 3);
        unsafe {
            let result = dfa.next_unchecked(0, 1);
            assert_eq!(result, 2);
        }
    }

    #[should_panic]
    fn test_next_unchecked_invalid_si() {
        let dfa = TestDFA::new(vec![1, 2, 3], 3);
        unsafe {
            let _result = dfa.next_unchecked(3, 1); // si == table.len()
        }
    }

    #[should_panic]
    fn test_next_unchecked_invalid_cls() {
        let dfa = TestDFA::new(vec![1, 2, 3], 3);
        unsafe {
            let _result = dfa.next_unchecked(1, 3); // cls == num_byte_classes
        }
    }
}

