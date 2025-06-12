// Answer 0

#[test]
fn test_exec_function() {
    struct MockProg {
        is_anchored_start: bool,
        prefixes: Vec<String>,
        matches: Vec<String>,
    }

    struct MockInput {
        current_pos: usize,
        input_data: String,
    }

    struct Threads {
        set: Vec<usize>,
    }

    struct Slot {
        data: String,
    }

    struct Mock {
        prog: MockProg,
        input: MockInput,
    }

    impl MockInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt { pos, size: self.input_data.len() }
        }
        
        fn prefix_at(&self, prefixes: &Vec<String>, at: InputAt) -> Option<InputAt> {
            for prefix in prefixes {
                if self.input_data[at.pos..].starts_with(prefix) {
                    return Some(at);
                }
            }
            None
        }
    }
    
    struct InputAt {
        pos: usize,
        size: usize,
    }

    // Simulating the method logic
    impl Mock {
        fn add(&self, clist: &mut Threads, slots: &mut Vec<Slot>, _: usize, at: InputAt) {
            clist.set.push(at.pos);
            slots.push(Slot { data: self.input.input_data[at.pos..].to_string() });
        }

        fn step(&self, nlist: &mut Threads, matches: &mut [bool], slots: &mut Vec<Slot>, _: &usize, _: usize, _: InputAt, _: InputAt) -> bool {
            matches[0] = true; // simulate a successful match
            nlist.set.push(1); // add a new thread to nlist
            true
        }

        fn exec_(
            &mut self,
            clist: &mut Threads,
            nlist: &mut Threads,
            matches: &mut [bool],
            slots: &mut Vec<Slot>,
            quit_after_match: bool,
            mut at: InputAt,
        ) -> bool {
            let mut matched = false;
            let mut all_matched = false;
            clist.set.clear();
            nlist.set.clear();

            'LOOP: loop {
                if clist.set.is_empty() {
                    if matched && matches.len() <= 1 || all_matched || (!at.is_start() && self.prog.is_anchored_start) {
                        break;
                    }
                    if !self.prog.prefixes.is_empty() {
                        at = match self.input.prefix_at(&self.prog.prefixes, at) {
                            None => break,
                            Some(at) => at,
                        };
                    }
                }

                if clist.set.is_empty() || (!self.prog.is_anchored_start && !all_matched) {
                    self.add(clist, slots, 0, at);
                }
                let at_next = self.input.at(at.pos + 1);
                for i in 0..clist.set.len() {
                    let ip = clist.set[i];
                    if self.step(nlist, matches, slots, &ip, ip, at, at_next) {
                        matched = true;
                        all_matched = all_matched || matches.iter().all(|&b| b);
                        if quit_after_match {
                            break 'LOOP;
                        }
                        if self.prog.matches.len() == 1 {
                            break;
                        }
                    }
                }
                if at.pos >= self.input.input_data.len() {
                    break;
                }
                at = at_next;
                std::mem::swap(clist, nlist);
                nlist.set.clear();
            }
            matched
        }
    }

    // Test inputs
    let mut threads = Threads { set: vec![0] };
    let mut nlist = Threads { set: Vec::new() };
    let mut matches = vec![false];
    let mut slots = Vec::new();

    let mock_prog = MockProg {
        is_anchored_start: false,
        prefixes: vec!["pre".to_string()],
        matches: vec!["match".to_string()],
    };

    let mock_input = MockInput {
        current_pos: 0,
        input_data: "prefix_data".to_string(),
    };

    let mut mock = Mock { prog: mock_prog, input: mock_input };

    let at_start = mock.input.at(0);
    let result = mock.exec_(&mut threads, &mut nlist, &mut matches, &mut slots, false, at_start);

    assert!(result); // Expect that exec_ returns true
}

