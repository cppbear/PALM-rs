// Answer 0

#[inline]
struct TestProg {
    items: Vec<prog::Inst>,
}

struct TestThreads {
    set: std::collections::HashSet<usize>,
    caps: Vec<Vec<Option<usize>>>,
}

impl TestThreads {
    fn new() -> Self {
        Self {
            set: std::collections::HashSet::new(),
            caps: vec![vec![None; 2]], // 2 slots for the sake of example
        }
    }
    
    fn caps(&mut self, index: usize) -> &mut Vec<Option<usize>> {
        &mut self.caps[index]
    }
}

#[test]
fn test_add_step_no_previous_visit() {
    let mut prog = TestProg {
        items: vec![
            prog::Inst::EmptyLook(prog::EmptyLook { goto: 1 }),
            prog::Inst::Ranges(prog::Ranges {}),
            prog::Inst::Match(prog::Match {}),
        ],
    };

    let mut nlist = TestThreads::new();
    let mut thread_caps = vec![None; 2];
    let mut ip = 0usize; // Starting at the first instruction
    let at = InputAt::new(0); // Simulated position

    // Ensure nlist.set does not contain the ip before the call
    assert!(!nlist.set.contains(ip));
    
    // Running the function
    add_step(&mut prog, &mut nlist, &mut thread_caps, ip, at.clone());

    // Assert that the value has been captured
    assert_eq!(nlist.caps(1), &vec![Some(0), None]); // Check that slot 0 was filled
}

#[test]
#[should_panic]
fn test_add_step_panic_slot_out_of_bounds() {
    let mut prog = TestProg {
        items: vec![
            prog::Inst::Save(prog::Save { slot: 3, goto: 1 }), // Slot 3 is out of bounds for thread_caps
            prog::Inst::Match(prog::Match {}),
        ],
    };

    let mut nlist = TestThreads::new();
    let mut thread_caps = vec![None; 2]; // Only 2 slots
    let mut ip = 0usize; // Starting at the first instruction
    let at = InputAt::new(0); // Simulated position

    // Running the function, it should panic
    add_step(&mut prog, &mut nlist, &mut thread_caps, ip, at);
}

#[test]
fn test_add_step_no_capture() {
    let mut prog = TestProg {
        items: vec![
            prog::Inst::Ranges(prog::Ranges {}),
            prog::Inst::Match(prog::Match {}),
        ],
    };

    let mut nlist = TestThreads::new();
    let mut thread_caps = vec![None; 2];
    let mut ip = 0usize; // Pointing to the Ranges instruction
    let at = InputAt::new(0); // Simulated position

    // Ensure no previous captures exist
    assert_eq!(nlist.caps(0), &mut vec![None, None]);
    
    // Running the function
    add_step(&mut prog, &mut nlist, &mut thread_caps, ip, at.clone());

    // Assert that captures remain unchanged
    assert_eq!(nlist.caps(0), &mut vec![None, None]); // No capture should have been made
}

