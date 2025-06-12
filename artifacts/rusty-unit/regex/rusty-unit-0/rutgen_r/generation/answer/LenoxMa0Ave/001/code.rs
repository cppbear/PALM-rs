// Answer 0

#[test]
fn test_reset_size_empty_states_and_stack() {
    struct TestDFA {
        start_states: Vec<i32>,
        stack: Vec<i32>,
        size: usize,
    }

    impl TestDFA {
        fn new() -> Self {
            TestDFA {
                start_states: Vec::new(),
                stack: Vec::new(),
                size: 0,
            }
        }

        fn reset_size(&mut self) {
            self.size =
                (self.start_states.len() * std::mem::size_of::<i32>())
                + (self.stack.len() * std::mem::size_of::<i32>());
        }
    }

    let mut dfa = TestDFA::new();
    dfa.reset_size();
    assert_eq!(dfa.size, 0);
}

#[test]
fn test_reset_size_with_one_start_state_and_no_stack() {
    struct TestDFA {
        start_states: Vec<i32>,
        stack: Vec<i32>,
        size: usize,
    }

    impl TestDFA {
        fn new(start_states: Vec<i32>, stack: Vec<i32>) -> Self {
            TestDFA {
                start_states,
                stack,
                size: 0,
            }
        }

        fn reset_size(&mut self) {
            self.size =
                (self.start_states.len() * std::mem::size_of::<i32>())
                + (self.stack.len() * std::mem::size_of::<i32>());
        }
    }

    let mut dfa = TestDFA::new(vec![1], Vec::new());
    dfa.reset_size();
    assert_eq!(dfa.size, std::mem::size_of::<i32>());
}

#[test]
fn test_reset_size_with_multiple_start_states_and_stack() {
    struct TestDFA {
        start_states: Vec<i32>,
        stack: Vec<i32>,
        size: usize,
    }

    impl TestDFA {
        fn new(start_states: Vec<i32>, stack: Vec<i32>) -> Self {
            TestDFA {
                start_states,
                stack,
                size: 0,
            }
        }

        fn reset_size(&mut self) {
            self.size =
                (self.start_states.len() * std::mem::size_of::<i32>())
                + (self.stack.len() * std::mem::size_of::<i32>());
        }
    }

    let mut dfa = TestDFA::new(vec![1, 2, 3], vec![10, 20]);
    dfa.reset_size();
    assert_eq!(dfa.size, 3 * std::mem::size_of::<i32>() + 2 * std::mem::size_of::<i32>());
} 

#[test]
fn test_reset_size_with_large_size() {
    struct TestDFA {
        start_states: Vec<i32>,
        stack: Vec<i32>,
        size: usize,
    }

    impl TestDFA {
        fn new(start_states: Vec<i32>, stack: Vec<i32>) -> Self {
            TestDFA {
                start_states,
                stack,
                size: 0,
            }
        }

        fn reset_size(&mut self) {
            self.size =
                (self.start_states.len() * std::mem::size_of::<i32>())
                + (self.stack.len() * std::mem::size_of::<i32>());
        }
    }

    let large_start_states = (0..1000).map(|x| x).collect::<Vec<i32>>();
    let large_stack = (0..500).map(|x| x).collect::<Vec<i32>>();
    
    let mut dfa = TestDFA::new(large_start_states, large_stack);
    dfa.reset_size();
    assert_eq!(dfa.size, 1000 * std::mem::size_of::<i32>() + 500 * std::mem::size_of::<i32>());
}

