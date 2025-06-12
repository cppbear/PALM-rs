// Answer 0

#[derive(Debug)]
struct Ast {
    value: String,
}

struct Formatter {
    output: String,
}

impl fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

#[test]
fn test_print_with_string_writer() {
    let ast = Ast { value: String::from("test") };
    let mut formatter = Formatter { output: String::new() };
    
    let result = print(&mut formatter, &ast);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "test");
}

#[test]
fn test_print_with_formatter() {
    let ast = Ast { value: String::from("example") };
    let mut formatter = Formatter { output: String::new() };
    
    let result = print(&mut formatter, &ast);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "example");
}

