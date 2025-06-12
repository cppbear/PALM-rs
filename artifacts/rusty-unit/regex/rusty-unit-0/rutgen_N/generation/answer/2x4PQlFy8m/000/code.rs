// Answer 0

#[test]
fn test_exec_single_match() {
    struct TestProg {
        is_anchored_start: bool,
        prefixes: Vec<u8>,
        matches: Vec<u8>,
    }
    
    struct TestInputAt {
        position: usize,
    }

    struct TestInput {
        data: Vec<u8>,
    }

    struct Threads {
        set: Vec<usize>,
    }

    struct Slot;

    impl TestInputAt {
        fn is_start(&self) -> bool {
            self.position == 0
        }
        
        fn is_end(&self) -> bool {
            self.position == 4 // assuming input length is 4
        }
        
        fn next_pos(&self) -> Self {
            TestInputAt {
                position: self.position + 1
            }
        }
    }

    struct Machine {
        prog: TestProg,
        input: TestInput,
    }

    impl Machine {
        fn add(&self, clist: &mut Threads, slots: &mut [Slot], _: usize, at: TestInputAt) {
            clist.set.push(at.position);
        }

        fn step(
            &self,
            nlist: &mut Threads,
            matches: &mut [bool],
            _: &mut [Slot],
            _: usize,
            _: usize,
            _: TestInputAt,
            _: TestInputAt,
        ) -> bool {
            matches[0] = true; // simulate a successful match
            nlist.set.push(0); // add to the next list
            true
        }
    }

    let mut clist = Threads { set: vec![] };
    let mut nlist = Threads { set: vec![] };
    let mut matches = vec![false];
    let mut slots: Vec<Slot> = vec![Slot];
    let quit_after_match = false;
    let at = TestInputAt { position: 0 };
    let mut machine = Machine {
        prog: TestProg {
            is_anchored_start: true,
            prefixes: vec![],
            matches: vec![b'a'],
        },
        input: TestInput {
            data: b"abcd".to_vec(),
        },
    };

    let result = machine.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, quit_after_match, at);
    assert!(result);
    assert!(matches[0]);
}

// Additional test function for edge case
#[test]
fn test_exec_no_match() {
    struct TestProg {
        is_anchored_start: bool,
        prefixes: Vec<u8>,
        matches: Vec<u8>,
    }

    struct TestInputAt {
        position: usize,
    }

    struct TestInput {
        data: Vec<u8>,
    }

    struct Threads {
        set: Vec<usize>,
    }

    struct Slot;

    impl TestInputAt {
        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.position == 4 // assuming input length is 4
        }

        fn next_pos(&self) -> Self {
            TestInputAt {
                position: self.position + 1,
            }
        }
    }

    struct Machine {
        prog: TestProg,
        input: TestInput,
    }

    impl Machine {
        fn add(&self, clist: &mut Threads, slots: &mut [Slot], _: usize, at: TestInputAt) {
            clist.set.push(at.position);
        }

        fn step(
            &self,
            nlist: &mut Threads,
            matches: &mut [bool],
            _: &mut [Slot],
            _: usize,
            _: usize,
            _: TestInputAt,
            _: TestInputAt,
        ) -> bool {
            false // simulate no match
        }
    }

    let mut clist = Threads { set: vec![] };
    let mut nlist = Threads { set: vec![] };
    let mut matches = vec![false];
    let mut slots: Vec<Slot> = vec![Slot];
    let quit_after_match = false;
    let at = TestInputAt { position: 0 };
    let mut machine = Machine {
        prog: TestProg {
            is_anchored_start: true,
            prefixes: vec![],
            matches: vec![b'z'], // no corresponding match
        },
        input: TestInput {
            data: b"abcd".to_vec(),
        },
    };

    let result = machine.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, quit_after_match, at);
    assert!(!result);
    assert!(!matches[0]);
}

