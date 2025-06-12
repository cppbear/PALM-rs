// Answer 0

#[test]
fn test_only_utf8_true() {
    let program = Program {
        only_utf8: true,
        ..Program::new()
    };
    program.only_utf8();
}

#[test]
fn test_only_utf8_false() {
    let program = Program {
        only_utf8: false,
        ..Program::new()
    };
    program.only_utf8();
}

#[test]
fn test_only_utf8_edge_case() {
    let program = Program {
        only_utf8: true,
        ..Program::new()
    };
    program.only_utf8();
    
    let program_edge = Program {
        only_utf8: false,
        ..Program::new()
    };
    program_edge.only_utf8();
}

