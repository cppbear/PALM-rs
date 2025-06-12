// Answer 0

#[test]
fn test_captures_len_no_captures() {
    struct TestExec {
        ro: Arc<ExecReadOnly>, // This would need to be properly initialized in real code.
    }

    let test_exec = TestExec {
        ro: Arc::new(ExecReadOnly {
            nfa: Nfa { captures: vec![] },
        }),
    };

    let regex = Regex(test_exec);
    assert_eq!(regex.captures_len(), 0);
}

#[test]
fn test_captures_len_with_captures() {
    struct TestExec {
        ro: Arc<ExecReadOnly>,
    }

    let test_exec = TestExec {
        ro: Arc::new(ExecReadOnly {
            nfa: Nfa {
                captures: vec![Some("capture1".to_string()), Some("capture2".to_string())],
            },
        }),
    };

    let regex = Regex(test_exec);
    assert_eq!(regex.captures_len(), 2);
}

