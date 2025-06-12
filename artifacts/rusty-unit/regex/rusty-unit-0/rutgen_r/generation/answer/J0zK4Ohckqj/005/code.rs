// Answer 0

#[test]
fn test_notate_empty_pattern() {
    struct Dummy {
        pattern: String,
        line_number_width: usize,
    }

    impl Dummy {
        fn left_pad_line_number(&self, line_number: usize) -> String {
            format!("{:width$}", line_number, width = self.line_number_width)
        }

        fn notate_line(&self, _line: usize) -> Option<String> {
            None
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
    }

    let dummy = Dummy {
        pattern: String::new(),
        line_number_width: 0,
    };
    let result = dummy.notate();
    assert_eq!(result, "\n");
}

#[test]
fn test_notate_single_line_no_numbers() {
    struct Dummy {
        pattern: String,
        line_number_width: usize,
    }

    impl Dummy {
        fn left_pad_line_number(&self, line_number: usize) -> String {
            format!("{:width$}", line_number, width = self.line_number_width)
        }

        fn notate_line(&self, _line: usize) -> Option<String> {
            None
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
    }

    let dummy = Dummy {
        pattern: "Hello, World!".to_string(),
        line_number_width: 0,
    };
    let result = dummy.notate();
    assert_eq!(result, "    Hello, World!\n");
}

#[test]
fn test_notate_multiple_lines_no_numbers() {
    struct Dummy {
        pattern: String,
        line_number_width: usize,
    }

    impl Dummy {
        fn left_pad_line_number(&self, line_number: usize) -> String {
            format!("{:width$}", line_number, width = self.line_number_width)
        }

        fn notate_line(&self, _line: usize) -> Option<String> {
            None
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
    }

    let dummy = Dummy {
        pattern: "First line.\nSecond line.".to_string(),
        line_number_width: 0,
    };
    let result = dummy.notate();
    assert_eq!(result, "    First line.\n    Second line.\n");
}

#[test]
fn test_notate_single_line_with_numbers() {
    struct Dummy {
        pattern: String,
        line_number_width: usize,
    }

    impl Dummy {
        fn left_pad_line_number(&self, line_number: usize) -> String {
            format!("{:width$}", line_number, width = self.line_number_width)
        }

        fn notate_line(&self, _line: usize) -> Option<String> {
            None
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
    }

    let dummy = Dummy {
        pattern: "Hello, World!".to_string(),
        line_number_width: 4,
    };
    let result = dummy.notate();
    assert_eq!(result, "   1: Hello, World!\n");
}

