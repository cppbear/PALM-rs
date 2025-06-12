// Answer 0

#[test]
fn test_from_formatter_with_single_line_pattern_and_aux_span() {
    struct Formatter<'e, E> {
        pattern: &'e str,
        span: String,
        aux_span: Option<String>,
    }

    struct Spans<'p> {
        pattern: &'p str,
        line_number_width: usize,
        by_line: Vec<Vec<String>>,
        multi_line: Vec<String>,
    }

    impl<'p> Spans<'p> {
        fn add(&mut self, span: String) {
            self.by_line[0].push(span);
        }
    }

    let pattern = "test pattern\n";
    let span = String::from("span data");
    let aux_span = Some(String::from("aux span data"));

    let fmter = Formatter {
        pattern,
        span,
        aux_span,
    };

    let result = from_formatter(&fmter);

    assert_eq!(result.pattern, pattern);
    assert_eq!(result.line_number_width, 0);
    assert_eq!(result.by_line.len(), 1);
    assert!(!result.by_line[0].is_empty());
    assert_eq!(result.by_line[0][0], span);
    assert_eq!(result.multi_line.len(), 0);
}

