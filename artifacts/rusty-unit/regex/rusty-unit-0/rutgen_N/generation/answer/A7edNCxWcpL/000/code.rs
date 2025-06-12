// Answer 0

#[test]
fn test_exec_match_found() {
    struct Program {
        len: usize,
        captures: Vec<usize>,
    }

    struct ProgramCache {
        pikevm: PikeVmCache,
    }

    struct PikeVmCache {
        clist: Vec<usize>,
        nlist: Vec<usize>,
        stack: Vec<usize>,
    }

    struct MockInput {
        data: Vec<char>,
    }

    impl MockInput {
        fn at(&self, index: usize) -> char {
            self.data[index]
        }
    }

    struct Slot;

    let prog = Program {
        len: 10,
        captures: vec![0, 1],
    };

    let mut cache = ProgramCache {
        pikevm: PikeVmCache {
            clist: Vec::new(),
            nlist: Vec::new(),
            stack: Vec::new(),
        },
    };

    let mut matches = vec![false; prog.captures.len()];
    let mut slots = vec![Slot; prog.captures.len()];

    let input = MockInput {
        data: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
    };

    let result = exec(&prog, &cache, &mut matches, &mut slots, true, input, 0);
    assert!(result);
}

#[test]
fn test_exec_no_match() {
    struct Program {
        len: usize,
        captures: Vec<usize>,
    }

    struct ProgramCache {
        pikevm: PikeVmCache,
    }

    struct PikeVmCache {
        clist: Vec<usize>,
        nlist: Vec<usize>,
        stack: Vec<usize>,
    }

    struct MockInput {
        data: Vec<char>,
    }

    impl MockInput {
        fn at(&self, index: usize) -> char {
            self.data[index]
        }
    }

    struct Slot;

    let prog = Program {
        len: 10,
        captures: vec![0, 1],
    };

    let mut cache = ProgramCache {
        pikevm: PikeVmCache {
            clist: Vec::new(),
            nlist: Vec::new(),
            stack: Vec::new(),
        },
    };

    let mut matches = vec![false; prog.captures.len()];
    let mut slots = vec![Slot; prog.captures.len()];

    let input = MockInput {
        data: vec!['x', 'y', 'z'],
    };

    let result = exec(&prog, &cache, &mut matches, &mut slots, true, input, 0);
    assert!(!result);
}

