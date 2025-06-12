// Answer 0

#[test]
fn test_exec_match_found() {
    struct MockInput {
        data: Vec<char>,
    }

    impl MockInput {
        fn at(&self, index: usize) -> char {
            self.data[index]
        }
    }

    struct Program {
        len: usize,
        captures: Vec<usize>,
    }

    struct ProgramCache {
        pikevm: PikevmCache,
    }

    struct PikevmCache {
        clist: Vec<usize>,
        nlist: Vec<usize>,
        stack: Vec<usize>,
    }

    struct Slot;

    let prog = Program { len: 3, captures: vec![0, 1] };
    let mut cache = ProgramCache { pikevm: PikevmCache { clist: vec![], nlist: vec![], stack: vec![] } };
    let mut matches = vec![false; prog.captures.len()];
    let mut slots = vec![Slot; prog.captures.len()];
    let input = MockInput { data: vec!['a', 'b', 'c'] };

    let result = exec(&prog, &cache, &mut matches, &mut slots, false, input, 0);
    assert!(result);
}

#[test]
fn test_exec_no_match() {
    struct MockInput {
        data: Vec<char>,
    }

    impl MockInput {
        fn at(&self, index: usize) -> char {
            self.data[index]
        }
    }

    struct Program {
        len: usize,
        captures: Vec<usize>,
    }

    struct ProgramCache {
        pikevm: PikevmCache,
    }

    struct PikevmCache {
        clist: Vec<usize>,
        nlist: Vec<usize>,
        stack: Vec<usize>,
    }

    struct Slot;

    let prog = Program { len: 3, captures: vec![0, 1] };
    let mut cache = ProgramCache { pikevm: PikevmCache { clist: vec![], nlist: vec![], stack: vec![] } };
    let mut matches = vec![false; prog.captures.len()];
    let mut slots = vec![Slot; prog.captures.len()];
    let input = MockInput { data: vec!['x', 'y', 'z'] };

    let result = exec(&prog, &cache, &mut matches, &mut slots, false, input, 0);
    assert!(!result);
}

#[test]
#[should_panic]
fn test_exec_out_of_bounds_start_index() {
    struct MockInput {
        data: Vec<char>,
    }

    impl MockInput {
        fn at(&self, index: usize) -> char {
            self.data[index]
        }
    }

    struct Program {
        len: usize,
        captures: Vec<usize>,
    }

    struct ProgramCache {
        pikevm: PikevmCache,
    }

    struct PikevmCache {
        clist: Vec<usize>,
        nlist: Vec<usize>,
        stack: Vec<usize>,
    }

    struct Slot;

    let prog = Program { len: 3, captures: vec![0, 1] };
    let mut cache = ProgramCache { pikevm: PikevmCache { clist: vec![], nlist: vec![], stack: vec![] } };
    let mut matches = vec![false; prog.captures.len()];
    let mut slots = vec![Slot; prog.captures.len()];
    let input = MockInput { data: vec!['a', 'b', 'c'] };

    exec(&prog, &cache, &mut matches, &mut slots, false, input, 5);
}

