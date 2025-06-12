// Answer 0

#[derive(Debug)]
struct Inst {
    goto: usize,
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<Inst>,
}

impl std::ops::Index<usize> for Instructions {
    type Output = Inst;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instructions[index]
    }
}

#[test]
fn test_skip_with_no_op_instruction() {
    let instructions = Instructions {
        instructions: vec![
            Inst { goto: 2 }, // Save instruction that redirects to index 2
            Inst { goto: 4 }, // Save instruction that redirects to index 4
            Inst { goto: 3 }, // Regular instruction, should return this index
            Inst { goto: 0 }, // No-op that redirects back to start
            Inst { goto: 5 }, // Save instruction that redirects to index 5
            Inst { goto: 6 }, // Regular instruction
        ],
    };

    let pc = 0; // Start at index 0, which is a no-op
    let result = skip(&instructions, pc);
    assert_eq!(result, 3); // Should skip to index 3
}

#[test]
fn test_skip_direct_to_regular_instruction() {
    let instructions = Instructions {
        instructions: vec![
            Inst { goto: 1 }, // Save instruction
            Inst { goto: 2 }, // Save instruction
            Inst { goto: 0 }, // Save instruction
            Inst { goto: 3 }, // Regular instruction
        ],
    };

    let pc = 0; // Start at index 0
    let result = skip(&instructions, pc);
    assert_eq!(result, 3); // Should skip through no-ops to index 3
}

#[test]
fn test_skip_no_no_op_instructions() {
    let instructions = Instructions {
        instructions: vec![
            Inst { goto: 5 }, // Regular instruction
            Inst { goto: 6 },
            Inst { goto: 7 },
            Inst { goto: 8 },
            Inst { goto: 9 },
            Inst { goto: 10 }, // Last regular instruction
        ],
    };

    let pc = 0; // Start at index 0
    let result = skip(&instructions, pc);
    assert_eq!(result, 0); // Should return the same index as no no-op is present
}

#[test]
fn test_skip_with_empty_instructions() {
    let instructions = Instructions { instructions: vec![] };

    let pc = 0; // Invalid index
    let result = skip(&instructions, pc);
    assert_eq!(result, 0); // Should handle gracefully (but realistically might panic or return an out-of-bounds error)
}

