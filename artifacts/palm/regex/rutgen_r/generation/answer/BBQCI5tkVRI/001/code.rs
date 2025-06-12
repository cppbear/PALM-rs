// Answer 0

#[derive(Debug)]
struct Threads;

impl Threads {
    pub fn new() -> Self {
        Threads
    }
}

#[derive(Debug)]
struct Cache {
    clist: Threads,
    nlist: Threads,
    stack: Vec<u8>,
}

#[derive(Debug)]
struct Program;

impl Program {
    pub fn new() -> Self {
        Program
    }
}

pub fn new(_prog: &Program) -> Cache {
    Cache {
        clist: Threads::new(),
        nlist: Threads::new(),
        stack: vec![],
    }
}

#[test]
fn test_new_cache() {
    let program = Program::new();
    let cache = new(&program);
    
    assert_eq!(format!("{:?}", cache.clist), format!("{:?}", Threads::new()));
    assert_eq!(format!("{:?}", cache.nlist), format!("{:?}", Threads::new()));
    assert_eq!(cache.stack, Vec::<u8>::new());
}

#[test]
fn test_new_cache_with_empty_program() {
    let program = Program::new();
    let cache = new(&program);
    
    assert!(cache.stack.is_empty());
    assert_eq!(format!("{:?}", cache.clist), format!("{:?}", Threads::new()));
    assert_eq!(format!("{:?}", cache.nlist), format!("{:?}", Threads::new()));
}

