// Answer 0

#[test]
fn test_exec_success() {
    struct DummyInput {
        data: &'static str,
    }

    impl DummyInput {
        fn at(&self, index: usize) -> char {
            self.data.chars().nth(index).unwrap()
        }
    }

    struct Program;

    struct ProgramCache {
        backtrack: bool,
    }

    struct Slot;

    let prog = &Program;
    let mut cache = ProgramCache { backtrack: true };
    let mut matches = [false; 10];
    let mut slots = [Slot; 10];
    let input = DummyInput { data: "abc" };
    
    let result = exec(prog, &mut cache, &mut matches, &mut slots, input, 0);
    
    assert!(result);
}

#[test]
fn test_exec_no_match() {
    struct DummyInput {
        data: &'static str,
    }

    impl DummyInput {
        fn at(&self, index: usize) -> char {
            self.data.chars().nth(index).unwrap()
        }
    }

    struct Program;

    struct ProgramCache {
        backtrack: bool,
    }

    struct Slot;

    let prog = &Program;
    let mut cache = ProgramCache { backtrack: false };
    let mut matches = [false; 10];
    let mut slots = [Slot; 10];
    let input = DummyInput { data: "xyz" };
    
    let result = exec(prog, &mut cache, &mut matches, &mut slots, input, 0);
    
    assert!(!result);
}

