// Answer 0

#[derive(Debug)]
struct Notater {
    pattern: String,
    line_number_width: usize,
}

impl Notater {
    fn left_pad_line_number(&self, number: usize) -> String {
        format!("{:>width$}", number, width = self.line_number_width)
    }

    fn notate_line(&self, _line: usize) -> Option<String> {
        Some("^")
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

#[test]
fn test_notate_with_multiple_lines() {
    let notater = Notater {
        pattern: String::from("first line\nsecond line\nthird line"),
        line_number_width: 3,
    };
    let result = notater.notate();
    let expected = "  1: first line\n^\n  2: second line\n^\n  3: third line\n^\n";
    assert_eq!(result, expected);
}

#[test]
fn test_notate_with_no_lines() {
    let notater = Notater {
        pattern: String::from(""),
        line_number_width: 3,
    };
    let result = notater.notate();
    let expected = "";
    assert_eq!(result, expected);
}

#[test]
fn test_notate_with_one_line() {
    let notater = Notater {
        pattern: String::from("only one line"),
        line_number_width: 3,
    };
    let result = notater.notate();
    let expected = "  1: only one line\n^\n";
    assert_eq!(result, expected);
}

#[test]
fn test_notate_with_zero_line_number_width() {
    let notater = Notater {
        pattern: String::from("single line"),
        line_number_width: 0,
    };
    let result = notater.notate();
    let expected = "    single line\n\n";
    assert_eq!(result, expected);
}

