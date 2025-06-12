// Answer 0

#[test]
fn test_state_fmt_with_empty_data() {
    let state = State {
        data: Box::new([0u8; 1]),
    };
    let mut output = vec![];
    let result = state.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "State { flags: StateFlags(0), insts: [] }");
}

#[test]
fn test_state_fmt_with_data() {
    let state = State {
        data: Box::new([1, 2, 3, 4]), // where 1 is the flag and 2, 3, 4 are instruction pointers
    };
    let mut output = vec![];
    let result = state.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "State { flags: StateFlags(1), insts: [2, 3, 4] }");
}

#[test]
fn test_state_fmt_with_maximum_data() {
    let state = State {
        data: Box::new([0; 1 + STATE_MAX as usize]),
    };
    let mut output = vec![];
    let result = state.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    let expected = format!("State {{ flags: StateFlags(0), insts: [{}] }}", repeat("0").take(STATE_MAX as usize).collect::<Vec<_>>().join(", "));
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}

