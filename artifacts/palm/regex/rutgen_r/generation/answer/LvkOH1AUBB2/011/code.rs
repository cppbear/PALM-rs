// Answer 0

#[test]
fn test_notate_line_with_non_empty_spans() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        column: usize,
    }

    struct TestStruct {
        by_line: Vec<Vec<Span>>,
    }

    impl TestStruct {
        fn line_number_padding(&self) -> usize {
            5 // Adjust this as necessary for testing
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
                for _ in 0..std::cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    // Set up test case
    let spans = vec![
        Span { start: Position { column: 3 }, end: Position { column: 5 } },
        Span { start: Position { column: 7 }, end: Position { column: 8 } },
    ];
    
    let test_struct = TestStruct {
        by_line: vec![spans], // Ensures there's at least one non-empty span
    };
    
    let result = test_struct.notate_line(0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "     ^^ ^");
}

#[test]
#[should_panic]
fn test_notate_line_with_non_existing_index() {
    struct Span {
        start: Position,
        end: Position,
    }

    struct Position {
        column: usize,
    }

    struct TestStruct {
        by_line: Vec<Vec<Span>>,
    }

    impl TestStruct {
        fn line_number_padding(&self) -> usize {
            0
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
                for _ in 0..std::cmp::max(1, note_len) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
    }

    // Set up test case with an empty by_line
    let test_struct = TestStruct {
        by_line: vec![],
    };
    
    // This should panic because the index 0 does not exist
    test_struct.notate_line(0);
}

