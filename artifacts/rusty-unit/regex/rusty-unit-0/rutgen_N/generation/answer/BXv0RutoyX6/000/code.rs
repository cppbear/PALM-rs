// Answer 0

#[derive(Debug)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

impl Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position(o: {:?}, l: {:?}, c: {:?})", self.offset, self.line, self.column)
    }
}

#[test]
fn test_fmt() {
    let pos = Position { offset: 5, line: 1, column: 3 };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        pos.fmt(formatter).unwrap();
    }
    assert_eq!(output, "Position(o: 5, l: 1, c: 3)");
}

#[test]
fn test_fmt_with_zeroes() {
    let pos = Position { offset: 0, line: 0, column: 0 };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        pos.fmt(formatter).unwrap();
    }
    assert_eq!(output, "Position(o: 0, l: 0, c: 0)");
}

#[test]
fn test_fmt_large_numbers() {
    let pos = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        pos.fmt(formatter).unwrap();
    }
    assert_eq!(output, format!("Position(o: {:?}, l: {:?}, c: {:?})", usize::MAX, usize::MAX, usize::MAX));
}

