// Answer 0

#[test]
fn test_step_with_save_instruction_and_visited() {
    use prog::{Inst, InstPtr, InstSave};
    use std::sync::Arc;
    use std::collections::HashMap;

    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char(self.data[i] as u32), byte: Some(self.data[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char(self.data[at.next_pos()] as u32)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(self.data[if at.pos == 0 { 0 } else { at.pos - 1 }] as u32)
        }

        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::Match(0),
        ],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let slots = &mut [Some(0)];
    let matches = &mut [false];
    let cache = Cache { jobs: vec![], visited: vec![0] };
    let input = TestInput { data: b"test".to_vec() };
    let start = 0;

    let mut bounded = Bounded {
        prog: &program,
        input,
        matches,
        slots,
        m: cache,
    };

    let result = bounded.step(0, InputAt { pos: 0, c: Char(0x74), byte: Some(0x74), len: 1 });
    assert_eq!(result, false);
}

