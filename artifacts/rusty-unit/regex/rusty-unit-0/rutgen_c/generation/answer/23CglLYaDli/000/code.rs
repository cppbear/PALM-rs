// Answer 0

#[test]
fn test_u8_class() {
    struct DummyInst;
    struct DummyProgram {
        byte_classes: Vec<u8>,
    }

    impl DummyProgram {
        fn new(byte_classes: Vec<u8>) -> Self {
            DummyProgram { byte_classes }
        }
    }
    
    let prog = DummyProgram::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };

    assert_eq!(fsm.u8_class(0), 0);
    assert_eq!(fsm.u8_class(1), 1);
    assert_eq!(fsm.u8_class(2), 2);
    assert_eq!(fsm.u8_class(3), 3);
    assert_eq!(fsm.u8_class(4), 4);
    assert_eq!(fsm.u8_class(5), 5);
    assert_eq!(fsm.u8_class(6), 6);
    assert_eq!(fsm.u8_class(7), 7);
    assert_eq!(fsm.u8_class(8), 8);
    assert_eq!(fsm.u8_class(9), 9);
}

#[test]
#[should_panic]
fn test_u8_class_out_of_bounds() {
    struct DummyInst;
    struct DummyProgram {
        byte_classes: Vec<u8>,
    }

    impl DummyProgram {
        fn new(byte_classes: Vec<u8>) -> Self {
            DummyProgram { byte_classes }
        }
    }

    let prog = DummyProgram::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);   
    let fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner {
            compiled: HashMap::new(),
            trans: Transitions::new(),
            states: Vec::new(),
            start_states: Vec::new(),
            stack: Vec::new(),
            flush_count: 0,
            size: 0,
        },
    };

    fsm.u8_class(10); // This should panic since 10 is out of bounds.
}

