// Answer 0

#[derive(Debug)]
struct Program {
    byte_classes: [u8; 256],
}

struct DFA {
    prog: Program,
}

impl DFA {
    fn u8_class(&self, b: u8) -> usize {
        self.prog.byte_classes[b as usize] as usize
    }
}

#[test]
fn test_u8_class_valid_input() {
    let prog = Program {
        byte_classes: [0; 256],
    };
    let dfa = DFA { prog };
    
    // Test with a mid-range value.
    let result = dfa.u8_class(100);
    assert_eq!(result, 0);
}

#[test]
fn test_u8_class_boundary_min() {
    let prog = Program {
        byte_classes: [1; 256],
    };
    let dfa = DFA { prog };

    // Test the minimum value
    let result = dfa.u8_class(0);
    assert_eq!(result, 1);
}

#[test]
fn test_u8_class_boundary_max() {
    let prog = Program {
        byte_classes: [2; 256],
    };
    let dfa = DFA { prog };

    // Test the maximum value
    let result = dfa.u8_class(255);
    assert_eq!(result, 2);
}

