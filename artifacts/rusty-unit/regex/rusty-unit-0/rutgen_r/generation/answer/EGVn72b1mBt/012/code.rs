// Answer 0

#[test]
fn test_step_match_condition() {
    struct DummyProg {
        prog: Vec<prog::Inst>,
    }

    struct Threads;

    struct Slot;

    struct InputAt {
        input: Vec<u8>,
        position: usize,
    }

    impl InputAt {
        fn char(&self) -> u8 {
            self.input[self.position]
        }

        fn byte(&self) -> Option<u8> {
            self.input.get(self.position).copied()
        }
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

    let mut prog = DummyProg {
        prog: vec![prog::Inst::Match(0)],
    };

    let mut nlist = Threads;
    let mut matches = vec![false];
    let mut slots = vec![Slot];
    let mut thread_caps = vec![None];
    let ip = 0;
    let at = InputAt {
        input: vec![b'a', b'b'],
        position: 0,
    };
    let at_next = InputAt {
        input: vec![b'a', b'b'],
        position: 1,
    };

    let result = prog.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, ip, at, at_next);
    
    assert!(result);
    assert!(matches[0]);
}

