// Answer 0

#[test]
fn test_fmt_with_non_empty_state() {
    struct TestState {
        flags: usize,
        inst_ptrs: Vec<usize>,
    }

    impl TestState {
        fn flags(&self) -> usize {
            self.flags
        }

        fn inst_ptrs(&self) -> std::slice::Iter<usize> {
            self.inst_ptrs.iter()
        }
    }

    let state = TestState {
        flags: 1,
        inst_ptrs: vec![0, 1, 2, 3],
    };

    let mut output = std::fmt::Formatter::new();
    state.fmt(&mut output).expect("Formatting should not panic");
}

#[test]
fn test_fmt_with_empty_state() {
    struct TestState {
        flags: usize,
        inst_ptrs: Vec<usize>,
    }

    impl TestState {
        fn flags(&self) -> usize {
            self.flags
        }

        fn inst_ptrs(&self) -> std::slice::Iter<usize> {
            self.inst_ptrs.iter()
        }
    }

    let state = TestState {
        flags: 0,
        inst_ptrs: vec![],
    };

    let mut output = std::fmt::Formatter::new();
    state.fmt(&mut output).expect("Formatting should not panic");
}

#[test]
fn test_fmt_with_large_state() {
    struct TestState {
        flags: usize,
        inst_ptrs: Vec<usize>,
    }

    impl TestState {
        fn flags(&self) -> usize {
            self.flags
        }

        fn inst_ptrs(&self) -> std::slice::Iter<usize> {
            self.inst_ptrs.iter()
        }
    }

    let state = TestState {
        flags: 2,
        inst_ptrs: (0..1000).collect(),
    };

    let mut output = std::fmt::Formatter::new();
    state.fmt(&mut output).expect("Formatting should not panic");
}

