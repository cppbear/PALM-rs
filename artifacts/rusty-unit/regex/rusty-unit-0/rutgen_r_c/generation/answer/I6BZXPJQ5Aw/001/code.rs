// Answer 0

#[test]
fn test_c_empty_look_start_line() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::StartLine);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_end_line() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::EndLine);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_start_text() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::StartText);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_end_text() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::EndText);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_word_boundary() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::WordBoundary);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_not_word_boundary() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::NotWordBoundary);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_word_boundary_ascii() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::WordBoundaryAscii);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

#[test]
fn test_c_empty_look_not_word_boundary_ascii() {
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::NotWordBoundaryAscii);
    match result {
        Ok(patch) => {
            assert_eq!(patch.hole, Hole::One(0));
            assert_eq!(patch.entry, 0);
        }
        _ => panic!("Expected Ok result"),
    }
}

