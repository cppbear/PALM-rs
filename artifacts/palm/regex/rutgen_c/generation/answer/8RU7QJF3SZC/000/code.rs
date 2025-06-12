// Answer 0

#[test]
fn test_debug_fmt_empty_transitions() {
    let transitions: Transitions = Transitions {
        table: Vec::new(),
        num_byte_classes: 0,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", transitions);
    assert!(result.is_ok());
    assert_eq!(output, "{}");
}

#[test]
fn test_debug_fmt_single_state() {
    let transitions: Transitions = Transitions {
        table: vec![0],
        num_byte_classes: 1,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", transitions);
    assert!(result.is_ok());
    assert_eq!(output, "{0: TransitionsRow([0])}");
}

#[test]
fn test_debug_fmt_multiple_states() {
    let transitions: Transitions = Transitions {
        table: vec![1, 2, 3, 4],
        num_byte_classes: 2,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", transitions);
    assert!(result.is_ok());
    assert_eq!(output, "{0: TransitionsRow([1, 2]), 1: TransitionsRow([3, 4])}");
}

