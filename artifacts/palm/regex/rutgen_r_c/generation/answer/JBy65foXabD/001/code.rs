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
    assert_eq!(program.approximate_size(), 
               (program.len() * std::mem::size_of::<Inst>()) +
               (program.matches.len() * std::mem::size_of::<InstPtr>()));
}

#[test]
fn test_approximate_size_with_captures() {
    let mut program = Program::new();
    program.captures.push(Some("capture1".to_string()));
    assert_eq!(program.approximate_size(), 
               (program.len() * std::mem::size_of::<Inst>()) +
               (program.matches.len() * std::mem::size_of::<InstPtr>()) +
               (program.captures.len() * std::mem::size_of::<Option<String>>()));
}

#[test]
fn test_approximate_size_with_capture_name_index() {
    let mut program = Program::new();
    program.capture_name_idx = Arc::new(HashMap::from([("name".to_string(), 0)]));
    assert_eq!(program.approximate_size(), 
               (program.len() * std::mem::size_of::<Inst>()) +
               (program.matches.len() * std::mem::size_of::<InstPtr>()) +
               (program.captures.len() * std::mem::size_of::<Option<String>>()) +
               (program.capture_name_idx.len() * 
                (std::mem::size_of::<String>() + std::mem::size_of::<usize>())));
}

#[test]
fn test_approximate_size_with_byte_classes() {
    let mut program = Program::new();
    program.byte_classes.extend_from_slice(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(program.approximate_size(), 
               (program.len() * std::mem::size_of::<Inst>()) +
               (program.matches.len() * std::mem::size_of::<InstPtr>()) +
               (program.captures.len() * std::mem::size_of::<Option<String>>()) +
               (program.capture_name_idx.len() * 
                (std::mem::size_of::<String>() + std::mem::size_of::<usize>())) +
               (program.byte_classes.len() * std::mem::size_of::<u8>()));
}

#[test]
fn test_approximate_size_with_prefixes() {
    let mut program = Program::new();
    program.prefixes = LiteralSearcher::empty(); // No heap allocation from prefixes
    let prefixes_size = program.prefixes.approximate_size();
    assert_eq!(program.approximate_size(), 
               (program.len() * std::mem::size_of::<Inst>()) +
               (program.matches.len() * std::mem::size_of::<InstPtr>()) +
               (program.captures.len() * std::mem::size_of::<Option<String>>()) +
               (program.capture_name_idx.len() * 
                (std::mem::size_of::<String>() + std::mem::size_of::<usize>())) +
               (program.byte_classes.len() * std::mem::size_of::<u8>()) +
               prefixes_size);
}

