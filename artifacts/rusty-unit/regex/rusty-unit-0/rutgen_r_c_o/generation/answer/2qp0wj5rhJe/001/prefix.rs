// Answer 0

#[test]
fn test_captures_len_with_no_captures() {
    let exec = Exec { 
        ro: Arc::new(ExecReadOnly { nfa: NFA { captures: vec![] } }), 
        cache: Default::default() 
    };
    let regex = Regex(exec);
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_with_one_capture() {
    let exec = Exec { 
        ro: Arc::new(ExecReadOnly { nfa: NFA { captures: vec![Some("first".to_string())] } }), 
        cache: Default::default() 
    };
    let regex = Regex(exec);
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_with_multiple_captures() {
    let exec = Exec { 
        ro: Arc::new(ExecReadOnly { nfa: NFA { captures: vec![Some("first".to_string()), Some("second".to_string())] } }), 
        cache: Default::default() 
    };
    let regex = Regex(exec);
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_with_maximum_captures() {
    let mut captures = Vec::new();
    for i in 0..10 {
        captures.push(Some(format!("capture{}", i)));
    }
    let exec = Exec { 
        ro: Arc::new(ExecReadOnly { nfa: NFA { captures } }), 
        cache: Default::default() 
    };
    let regex = Regex(exec);
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_with_none_captures() {
    let exec = Exec { 
        ro: Arc::new(ExecReadOnly { nfa: NFA { captures: vec![None] } }), 
        cache: Default::default() 
    };
    let regex = Regex(exec);
    let _ = regex.captures_len();
}

