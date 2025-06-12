// Answer 0

#[test]
fn test_step_match_slot_in_bounds() {
    struct DummyProg {
        prog: Vec<prog::Inst>,
    }

    impl DummyProg {
        fn step(
            &mut self,
            nlist: &mut Threads,
            matches: &mut [bool],
            slots: &mut [Slot],
            thread_caps: &mut [Option<usize>],
            ip: usize,
            at: InputAt,
            at_next: InputAt,
        ) -> bool {
            use prog::Inst::*;
            match self.prog[ip] {
                Match(match_slot) => {
                    if match_slot < matches.len() {
                        matches[match_slot] = true;
                    }
                    for (slot, val) in slots.iter_mut().zip(thread_caps.iter()) {
                        *slot = *val;
                    }
                    true
                }
                _ => false,
            }
        }
    }

    let mut nlist = Threads::new();
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![Some(0); 1];
    let ip = 0;
    let at = InputAt::new('a');
    let at_next = InputAt::new('b');
    
    let mut dummy_prog = DummyProg { prog: vec![prog::Inst::Match(0)] };

    let result = dummy_prog.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, ip, at, at_next);
    
    assert!(result);
    assert!(matches[0]);
}

#[test]
#[should_panic]
fn test_step_match_slot_out_of_bounds() {
    struct DummyProg {
        prog: Vec<prog::Inst>,
    }

    impl DummyProg {
        fn step(
            &mut self,
            nlist: &mut Threads,
            matches: &mut [bool],
            slots: &mut [Slot],
            thread_caps: &mut [Option<usize>],
            ip: usize,
            at: InputAt,
            at_next: InputAt,
        ) -> bool {
            use prog::Inst::*;
            match self.prog[ip] {
                Match(match_slot) => {
                    if match_slot >= matches.len() {
                        panic!("match_slot out of bounds!");
                    }
                    matches[match_slot] = true;
                    for (slot, val) in slots.iter_mut().zip(thread_caps.iter()) {
                        *slot = *val;
                    }
                    true
                }
                _ => false,
            }
        }
    }

    let mut nlist = Threads::new();
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![Some(0); 1];
    let ip = 0;
    let at = InputAt::new('a');
    let at_next = InputAt::new('b');
    
    let mut dummy_prog = DummyProg { prog: vec![prog::Inst::Match(1)] }; // match_slot == matches.len()

    dummy_prog.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, ip, at, at_next);
}

#[test]
fn test_step_slots_and_thread_caps() {
    struct DummyProg {
        prog: Vec<prog::Inst>,
    }

    impl DummyProg {
        fn step(
            &mut self,
            nlist: &mut Threads,
            matches: &mut [bool],
            slots: &mut [Slot],
            thread_caps: &mut [Option<usize>],
            ip: usize,
            at: InputAt,
            at_next: InputAt,
        ) -> bool {
            use prog::Inst::*;
            match self.prog[ip] {
                Match(match_slot) => {
                    if match_slot < matches.len() {
                        matches[match_slot] = true;
                    }
                    for (slot, val) in slots.iter_mut().zip(thread_caps.iter()) {
                        *slot = *val;
                    }
                    true
                }
                _ => false,
            }
        }
    }

    let mut nlist = Threads::new();
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![None; 1];
    let ip = 0;
    let at = InputAt::new('a');
    let at_next = InputAt::new('b');
    
    let mut dummy_prog = DummyProg { prog: vec![prog::Inst::Match(0)] };

    let result = dummy_prog.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, ip, at, at_next);
    
    assert!(result);
    assert!(matches[0]);
    assert_eq!(slots[0], thread_caps[0]); // should be None
} 

#[test]
fn test_step_no_matches() {
    struct DummyProg {
        prog: Vec<prog::Inst>,
    }

    impl DummyProg {
        fn step(
            &mut self,
            nlist: &mut Threads,
            matches: &mut [bool],
            slots: &mut [Slot],
            thread_caps: &mut [Option<usize>],
            ip: usize,
            at: InputAt,
            at_next: InputAt,
        ) -> bool {
            use prog::Inst::*;
            match self.prog[ip] {
                Match(match_slot) => {
                    if match_slot < matches.len() {
                        matches[match_slot] = true;
                    }
                    for (slot, val) in slots.iter_mut().zip(thread_caps.iter()) {
                        *slot = *val;
                    }
                    true
                }
                _ => false,
            }
        }
    }

    let mut nlist = Threads::new();
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![Some(0); 1];
    let ip = 0;
    let at = InputAt::new('a');
    let at_next = InputAt::new('b');

    let mut dummy_prog = DummyProg { prog: vec![prog::Inst::EmptyLook(0)] };

    let result = dummy_prog.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, ip, at, at_next);
    
    assert!(!result);
    assert!(!matches[0]);
}

