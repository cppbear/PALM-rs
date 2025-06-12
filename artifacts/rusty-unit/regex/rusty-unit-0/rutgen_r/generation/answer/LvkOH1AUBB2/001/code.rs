// Answer 0

#[test]
fn test_notate_line_empty_spans() {
    struct TestStruct {
        by_line: Vec<Vec<Span>>,
    }

    #[derive(Default)]
    struct Span {
        start: Position,
        end: Position,
    }

    #[derive(Default)]
    struct Position {
        column: usize,
    }

    impl TestStruct {
        fn line_number_padding(&self) -> usize {
            0 // For simplicity in this test, we return 0
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_padding() {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..core::cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    let test_instance = TestStruct {
        by_line: vec![vec![], vec![]], // Line 0 contains empty spans
    };

    assert_eq!(test_instance.notate_line(0), None); // Testing for index 0
}

#[test]
#[should_panic]
fn test_notate_line_out_of_bounds() {
    struct TestStruct {
        by_line: Vec<Vec<Span>>,
    }

    #[derive(Default)]
    struct Span {
        start: Position,
        end: Position,
    }

    #[derive(Default)]
    struct Position {
        column: usize,
    }

    impl TestStruct {
        fn line_number_padding(&self) -> usize {
            0 // For simplicity in this test, we return 0
        }

        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i]; // May panic if i is out of bounds
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            for _ in 0..self.line_number_padding() {
                notes.push(' ');
            }
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..core::cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    let test_instance = TestStruct {
        by_line: vec![vec![], vec![]], // Line 0 contains empty spans
    };
    
    test_instance.notate_line(2); // This should panic due to out of bounds index
}

