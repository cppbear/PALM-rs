// Answer 0

#[derive(Default)]
struct Threads {
    set: std::collections::HashSet<usize>,
}

#[derive(Default)]
struct PikeVM {
    prog: Vec<Inst>,
    input: Input,
    stack: Vec<FollowEpsilon>,
}

enum Inst {
    EmptyLook(LookInst),
    Save(SaveInst),
    Split(SplitInst),
    Match(MatchInst),
    Char(CharInst),
    Ranges(RangesInst),
    Bytes(BytesInst),
}

struct LookInst {
    goto: usize,
}

struct SaveInst {
    slot: usize,
    goto: usize,
}

struct SplitInst {
    goto1: usize,
    goto2: usize,
}

struct MatchInst;
struct CharInst;
struct RangesInst;
struct BytesInst;

enum FollowEpsilon {
    Capture { slot: usize, pos: Option<usize> },
    IP(usize),
}

struct Input;

impl Input {
    fn is_empty_match(&self, _at: InputAt, _inst: &LookInst) -> bool {
        false
    }
}

struct InputAt {
    pos: usize,
}

impl InputAt {
    fn pos(&self) -> usize {
        self.pos
    }
}

#[test]
fn test_add_step_with_char_match() {
    let mut nlist = Threads::default();
    let mut thread_caps = vec![None; 1]; // one capture slot
    let mut vm = PikeVM {
        prog: vec![Inst::Char(CharInst), Inst::Match(MatchInst)],
        input: Input::default(),
        stack: vec![],
    };
    
    let ip = 0; // starting instruction pointer
    let at = InputAt { pos: 0 }; // input position
    
    vm.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(&ip)); // ensure the instruction pointer is added
    assert!(thread_caps[0].is_none()); // capture slot remains None
}

#[test]
fn test_add_step_with_no_previous_visit() {
    let mut nlist = Threads::default();
    let mut thread_caps = vec![None; 1];
    let mut vm = PikeVM {
        prog: vec![Inst::Char(CharInst)],
        input: Input::default(),
        stack: vec![],
    };
    
    let ip = 0;
    let at = InputAt { pos: 0 };
    
    vm.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(&ip));
    assert!(thread_caps[0].is_none());
}

#[test]
fn test_add_step_empty_look_falls_through() {
    let mut nlist = Threads::default();
    let mut thread_caps = vec![None; 1];
    let mut vm = PikeVM {
        prog: vec![Inst::EmptyLook(LookInst { goto: 1 }), Inst::Char(CharInst)],
        input: Input::default(),
        stack: vec![],
    };
    
    let ip = 0;
    let at = InputAt { pos: 0 };

    vm.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(&ip));
    assert!(thread_caps[0].is_none()); // ensure capture slot still None
}

#[test]
#[should_panic]
fn test_add_step_invalid_index_access() {
    let mut nlist = Threads::default();
    let mut thread_caps = vec![None; 1];
    let mut vm = PikeVM {
        prog: vec![Inst::Char(CharInst)], // only one instruction
        input: Input::default(),
        stack: vec![],
    };
    
    let ip = 1; // out of bounds instruction pointer
    let at = InputAt { pos: 0 };
    
    vm.add_step(&mut nlist, &mut thread_caps, ip, at); // should panic on out of bounds access
}

