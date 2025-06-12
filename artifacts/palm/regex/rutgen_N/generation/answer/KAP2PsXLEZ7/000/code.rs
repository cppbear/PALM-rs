// Answer 0

#[test]
fn test_cached_state_key_empty_state() {
    struct Dummy {
        prog: Vec<prog::Inst>,
    }

    impl Dummy {
        fn continue_past_first_match(&self) -> bool {
            false
        }
    }

    let mut instance = Dummy { prog: vec![] };
    let mut state_flags = StateFlags(0);
    let q = SparseSet::new();

    let result = instance.cached_state_key(&q, &mut state_flags);
    assert!(result.is_none());
}

#[test]
fn test_cached_state_key_match_state() {
    struct Dummy {
        prog: Vec<prog::Inst>,
    }

    impl Dummy {
        fn continue_past_first_match(&self) -> bool {
            true
        }
    }

    let mut instance = Dummy {
        prog: vec![prog::Inst::Match(0)],
    };
    let mut state_flags = StateFlags(1);
    let q = SparseSet::from_indices(&[0]);

    let result = instance.cached_state_key(&q, &mut state_flags);
    assert!(result.is_some());
    let state = result.unwrap();
    assert_eq!(state.data.len(), 1);
    assert_eq!(state.data[0], 1);
}

#[test]
fn test_cached_state_key_empty_look_state() {
    struct Dummy {
        prog: Vec<prog::Inst>,
    }

    impl Dummy {
        fn continue_past_first_match(&self) -> bool {
            false
        }
    }

    let mut instance = Dummy {
        prog: vec![prog::Inst::EmptyLook(0)],
    };
    let mut state_flags = StateFlags(0);
    let q = SparseSet::from_indices(&[0]);

    let result = instance.cached_state_key(&q, &mut state_flags);
    assert!(result.is_some());
    let state = result.unwrap();
    assert_eq!(state.data.len(), 1);
    assert_eq!(state.data[0], 0);
}

#[test]
fn test_cached_state_key_dead_state() {
    struct Dummy {
        prog: Vec<prog::Inst>,
    }

    impl Dummy {
        fn continue_past_first_match(&self) -> bool {
            false
        }
    }

    let mut instance = Dummy {
        prog: vec![prog::Inst::Save(0)],
    };
    let mut state_flags = StateFlags(0);
    let q = SparseSet::from_indices(&[0]);

    let result = instance.cached_state_key(&q, &mut state_flags);
    assert!(result.is_none());
}

