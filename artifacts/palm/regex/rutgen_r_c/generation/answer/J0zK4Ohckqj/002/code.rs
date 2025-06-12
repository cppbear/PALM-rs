// Answer 0

fn main() {
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Position {
        column: usize,
    }
    
    mod ast {
        use super::Position;
        #[derive(Clone)]
        pub struct Span {
            pub start: Position,
            pub end: Position,
        }
    }

    struct Spans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<ast::Span>>,
        multi_line: Vec<ast::Span>,
    }

    impl<'p> Spans<'p> {
        fn from_formatter<'e, E: std::fmt::Display>(_fmter: &'p ()) -> Self {
            todo!() // Stub
        }
        
        fn add(&mut self, _span: ast::Span) {
            todo!() // Stub
        }
        
        fn notate(&self) -> String {
            let mut notated = String::new();
            for (i, line) in self.pattern.lines().enumerate() {
                if self.line_number_width > 0 {
                    notated.push_str(&self.left_pad_line_number(i + 1));
                    notated.push_str(": ");
                } else {
                    notated.push_str("    ");
                }
                notated.push_str(line);
                notated.push('\n');
                if let Some(notes) = self.notate_line(i) {
                    notated.push_str(&notes);
                    notated.push('\n');
                }
            }
            notated
        }
        
        fn notate_line(&self, i: usize) -> Option<String> {
            let spans = &self.by_line[i];
            if spans.is_empty() {
                return None;
            }
            let mut notes = String::new();
            let mut pos = 0;
            for span in spans {
                for _ in pos..(span.start.column - 1) {
                    notes.push(' ');
                    pos += 1;
                }
                let note_len = span.end.column.saturating_sub(span.start.column);
                for _ in 0..note_len.max(1) {
                    notes.push('^');
                    pos += 1;
                }
            }
            Some(notes)
        }
        
        fn left_pad_line_number(&self, n: usize) -> String {
            let n = n.to_string();
            let pad = self.line_number_width.checked_sub(n.len()).unwrap();
            let mut result = " ".repeat(pad);
            result.push_str(&n);
            result
        }
    }

    #[test]
    fn test_notate_single_line_with_spans() {
        let pattern = "abc\ndef\nghi";
        let spans = vec![
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 2 } }],
            vec![],
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 4 } }],
        ];
        
        let spans_struct = Spans {
            pattern,
            line_number_width: 2,
            by_line: spans,
            multi_line: vec![],
        };

        let result = spans_struct.notate();
        assert_eq!(result, " 1: abc\n  ^\n 2: def\n 3: ghi\n ^^^\n");
    }

    #[test]
    fn test_notate_multiple_lines_with_spans() {
        let pattern = "hello\nworld\nrust";
        let spans = vec![
            vec![ast::Span { start: Position { column: 2 }, end: Position { column: 5 } }],
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 5 } }],
            vec![],
        ];
        
        let spans_struct = Spans {
            pattern,
            line_number_width: 2,
            by_line: spans,
            multi_line: vec![],
        };

        let result = spans_struct.notate();
        assert_eq!(result, " 1: hello\n  ^^^\n 2: world\n  ^^^^\n 3: rust\n");
    }

    #[test]
    fn test_notate_no_spans() {
        let pattern = "foo\nbar\nbaz";
        let spans = vec![
            vec![],
            vec![],
            vec![],
        ];
        
        let spans_struct = Spans {
            pattern,
            line_number_width: 2,
            by_line: spans,
            multi_line: vec![],
        };

        let result = spans_struct.notate();
        assert_eq!(result, " 1: foo\n 2: bar\n 3: baz\n");
    }

    #[test]
    fn test_notate_when_no_line_numbers() {
        let pattern = "test\ncase";
        let spans = vec![
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 4 } }],
            vec![ast::Span { start: Position { column: 1 }, end: Position { column: 4 } }],
        ];
        
        let spans_struct = Spans {
            pattern,
            line_number_width: 0,
            by_line: spans,
            multi_line: vec![],
        };

        let result = spans_struct.notate();
        assert_eq!(result, "    test\n    ^^^\n    case\n    ^^^\n");
    }
}

