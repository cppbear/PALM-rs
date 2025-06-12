// Answer 0

#[test]
fn test_add_multiline_span() {
    struct TestPosition {
        line: usize,
        column: usize,
    }

    struct TestSpan {
        start: TestPosition,
        end: TestPosition,
    }

    impl TestSpan {
        fn is_one_line(&self) -> bool {
            self.start.line == self.end.line
        }
    }

    struct TestAst {
        multi_line: Vec<TestSpan>,
        by_line: Vec<Vec<TestSpan>>,
    }

    impl TestAst {
        fn new() -> Self {
            TestAst {
                multi_line: Vec::new(),
                by_line: Vec::new(),
            }
        }

        fn add(&mut self, span: TestSpan) {
            if span.is_one_line() {
                let i = span.start.line - 1; // because lines are 1-indexed
                self.by_line[i].push(span);
                self.by_line[i].sort();
            } else {
                self.multi_line.push(span);
                self.multi_line.sort();
            }
        }
    }

    // Create an instance of the test structure
    let mut ast_instance = TestAst::new();
    
    // Create a multiline span that will trigger the specific condition
    let span = TestSpan {
        start: TestPosition { line: 1, column: 0 },
        end: TestPosition { line: 2, column: 5 },
    };

    // Call the add function with a multiline span
    ast_instance.add(span);

    // Assert the span is added to the multi_line collection
    assert_eq!(ast_instance.multi_line.len(), 1);
    assert_eq!(ast_instance.multi_line[0].start.line, 1);
    assert_eq!(ast_instance.multi_line[0].end.line, 2);
}

