// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDFA {
        start_states: Vec<usize>,
        stack: Vec<usize>,
        size: usize,
    }

    impl TestDFA {
        fn new() -> Self {
            Self {
                start_states: Vec::new(),
                stack: Vec::new(),
                size: 0,
            }
        }

        fn reset_size(&mut self) {
            self.size =
                (self.start_states.len() * std::mem::size_of::<usize>())
                + (self.stack.len() * std::mem::size_of::<usize>());
        }
    }

    #[test]
    fn test_reset_size_empty() {
        let mut dfa = TestDFA::new();
        dfa.reset_size();
        assert_eq!(dfa.size, 0);
    }

    #[test]
    fn test_reset_size_with_start_states() {
        let mut dfa = TestDFA::new();
        dfa.start_states.push(1);
        dfa.start_states.push(2);
        dfa.reset_size();
        assert_eq!(dfa.size, 2 * std::mem::size_of::<usize>());
    }

    #[test]
    fn test_reset_size_with_stack() {
        let mut dfa = TestDFA::new();
        dfa.stack.push(1);
        dfa.reset_size();
        assert_eq!(dfa.size, 0 * std::mem::size_of::<usize>() + 1 * std::mem::size_of::<usize>());
    }

    #[test]
    fn test_reset_size_with_both() {
        let mut dfa = TestDFA::new();
        dfa.start_states.push(1);
        dfa.stack.push(1);
        dfa.stack.push(2);
        dfa.reset_size();
        assert_eq!(dfa.size, 1 * std::mem::size_of::<usize>() + 2 * std::mem::size_of::<usize>());
    }
}

