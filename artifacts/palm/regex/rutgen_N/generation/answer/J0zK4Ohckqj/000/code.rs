// Answer 0

#[derive(Debug)]
struct NotateStruct {
    pattern: String,
    line_number_width: usize,
}

impl NotateStruct {
    fn left_pad_line_number(&self, number: usize) -> String {
        format!("{:width$}", number, width = self.line_number_width)
    }

    fn notate_line(&self, _line_index: usize) -> Option<String> {
        // For simplification, let's assume a basic notate outline.
        Some("^".repeat(self.pattern.lines().nth(_line_index)?.len()))
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
fn test_notate_with_line_numbers() {
    let notate_struct = NotateStruct {
        pattern: String::from("first line\nsecond line\nthird line"),
        line_number_width: 2,
    };
    assert_eq!(
        notate_struct.notate(),
        " 1: first line\n^^^^^^^^^^^^\n 2: second line\n^^^^^^^^^^^^^^\n 3: third line\n^^^^^^^^^^^\n"
    );
}

#[test]
fn test_notate_without_line_numbers() {
    let notate_struct = NotateStruct {
        pattern: String::from("only one line"),
        line_number_width: 0,
    };
    assert_eq!(
        notate_struct.notate(),
        "    only one line\n    ^^^^^^^^^^^^\n"
    );
}

#[test]
fn test_notate_empty() {
    let notate_struct = NotateStruct {
        pattern: String::from(""),
        line_number_width: 2,
    };
    assert_eq!(notate_struct.notate(), "\n");
}

