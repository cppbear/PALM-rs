// Answer 0

#[test]
fn test_state_fmt_with_empty_data() {
    let state = State {
        data: Box::new([]),
    };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| state.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "State { flags: StateFlags(0), insts: [] }");
}

#[test]
fn test_state_fmt_with_single_element_data() {
    let state = State {
        data: Box::new([5]),
    };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| state.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "State { flags: StateFlags(5), insts: [] }");
}

#[test]
fn test_state_fmt_with_multiple_elements_data() {
    let state = State {
        data: Box::new([2, 1, 2, 3]),  // flags = 2, inst_ptrs = [1, 2, 3]
    };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| state.fmt(f));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "State { flags: StateFlags(2), insts: [1, 2, 3] }");
}

#[test]
#[should_panic]
fn test_state_fmt_panic_on_invalid_data() {
    let state = State {
        data: Box::new([0u8; 0]), // No elements at all
    };
    let _ = std::fmt::write(&mut vec![], |f| state.fmt(f));
}

#[test]
fn test_state_fmt_with_large_data() {
    let data = vec![3u8].into_iter().chain(repeat(1).take(100)).collect::<Vec<u8>>(); // flags = 3, inst_ptrs = [1, 1, ..., 1]
    let state = State {
        data: data.into_boxed_slice(),
    };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| state.fmt(f));
    assert!(result.is_ok());
    let insts: String = repeat("1").take(100).collect::<Vec<&str>>().join(", ");
    assert_eq!(String::from_utf8(output).unwrap(), format!("State {{ flags: StateFlags(3), insts: [{}] }}", insts));
}

