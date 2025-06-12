// Answer 0

#[test]
fn test_c_class_single_character_range() {
    use syntax::hir::{ClassUnicodeRange};

    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = false; // Ensure uses_bytes() returns false

    let ranges = vec![ClassUnicodeRange::new('a', 'a')]; // Single character range
    let result = compiler.c_class(&ranges);

    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert_eq!(patch.entry, 0); // The entry is at the start
        // Ensure the hole is for character 'a'
        if let Hole::One(hole_index) = patch.hole {
            if let MaybeInst::Uncompiled(InstHole::Char { c }) = compiler.insts[hole_index] {
                assert_eq!(c, 'a');
            } else {
                panic!("Expected hole to be a Char instruction for 'a'");
            }
        } else {
            panic!("Expected hole to be of type One");
        }
    } else {
        panic!("Expected result to be Ok");
    }
}

