// Answer 0

#[test]
fn test_approximate_size_empty_program() {
    let program = Program::new();
    assert_eq!(program.approximate_size(), 0);
}

#[test]
fn test_approximate_size_with_matches() {
    let mut program = Program::new();
    program.matches.push(0);
    program.matches.push(1);
    assert_eq!(program.approximate_size(), mem::size_of::<Inst>() * 0 +
                                                  mem::size_of::<InstPtr>() * 2 +
                                                  mem::size_of::<Option<String>>() * 0 +
                                                  mem::size_of::<String>() * 0 +
                                                  mem::size_of::<usize>() * 0 +
                                                  mem::size_of::<u8>() * 256);
}

#[test]
fn test_approximate_size_with_captures() {
    let mut program = Program::new();
    program.captures.push(Some("first".to_string()));
    program.captures.push(Some("second".to_string()));
    program.capture_name_idx.insert("first".to_string(), 0);
    program.capture_name_idx.insert("second".to_string(), 1);
    assert_eq!(program.approximate_size(), mem::size_of::<Inst>() * 0 +
                                                  mem::size_of::<InstPtr>() * 0 +
                                                  mem::size_of::<Option<String>>() * 2 +
                                                  mem::size_of::<String>() * 15 + // approximate string sizes
                                                  mem::size_of::<usize>() * 2 +
                                                  mem::size_of::<u8>() * 256);
}

#[test]
fn test_approximate_size_with_byte_classes() {
    let mut program = Program::new();
    program.byte_classes[0] = 1; // Example modification to byte_classes
    program.byte_classes[255] = 1;
    assert_eq!(program.approximate_size(), mem::size_of::<Inst>() * 0 +
                                                  mem::size_of::<InstPtr>() * 0 +
                                                  mem::size_of::<Option<String>>() * 0 +
                                                  mem::size_of::<String>() * 0 +
                                                  mem::size_of::<usize>() * 0 +
                                                  mem::size_of::<u8>() * 256);
}

