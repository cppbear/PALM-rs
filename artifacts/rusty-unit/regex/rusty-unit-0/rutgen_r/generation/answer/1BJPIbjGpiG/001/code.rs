// Answer 0

fn fmt_literal_test() {
    use std::fmt::Write;
    use regex_syntax::ast::{self, Literal, SpecialLiteralKind};

    struct TestFormatter {
        wtr: String,
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter { wtr: String::new() }
        }
        
        fn fmt_literal(&mut self, ast: &Literal) -> std::fmt::Result {
            // Implementation of the fmt_literal function (as provided in the prompt).
            use ast::LiteralKind::*;

            match ast.kind {
                Special(SpecialLiteralKind::Bell) => {
                    self.wtr.write_str(r"\a")
                }
                Special(SpecialLiteralKind::FormFeed) => {
                    self.wtr.write_str(r"\f")
                }
                Special(SpecialLiteralKind::Tab) => {
                    self.wtr.write_str(r"\t")
                }
                Special(SpecialLiteralKind::LineFeed) => {
                    self.wtr.write_str(r"\n")
                }
                Special(SpecialLiteralKind::CarriageReturn) => {
                    self.wtr.write_str(r"\r")
                }
                Special(SpecialLiteralKind::VerticalTab) => {
                    self.wtr.write_str(r"\v")
                }
                Special(SpecialLiteralKind::Space) => {
                    self.wtr.write_str(r"\ ")
                }
                _ => unreachable!(),
            }
        }
    }

    let mut formatter = TestFormatter::new();

    // Test cases:

    // Test for Special Bell
    let bell_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::Bell), 
        c: 'a' 
    };
    formatter.fmt_literal(&bell_ast).unwrap();
    assert_eq!(formatter.wtr, r"\a");

    // Test for Special FormFeed
    let form_feed_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::FormFeed), 
        c: 'f' 
    };
    formatter.wtr.clear(); // Clear previous output
    formatter.fmt_literal(&form_feed_ast).unwrap();
    assert_eq!(formatter.wtr, r"\f");

    // Test for Special Tab
    let tab_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::Tab), 
        c: 't' 
    };
    formatter.wtr.clear(); // Clear previous output
    formatter.fmt_literal(&tab_ast).unwrap();
    assert_eq!(formatter.wtr, r"\t");

    // Test for Special Line Feed
    let line_feed_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::LineFeed), 
        c: 'n' 
    };
    formatter.wtr.clear(); // Clear previous output
    formatter.fmt_literal(&line_feed_ast).unwrap();
    assert_eq!(formatter.wtr, r"\n");

    // Test for Special Carriage Return
    let carriage_return_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::CarriageReturn), 
        c: 'r' 
    };
    formatter.wtr.clear(); // Clear previous output
    formatter.fmt_literal(&carriage_return_ast).unwrap();
    assert_eq!(formatter.wtr, r"\r");

    // Test for Special Vertical Tab
    let vertical_tab_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::VerticalTab), 
        c: 'v' 
    };
    formatter.wtr.clear(); // Clear previous output
    formatter.fmt_literal(&vertical_tab_ast).unwrap();
    assert_eq!(formatter.wtr, r"\v");

    // Test for Special Space
    let space_ast = Literal { 
        kind: ast::LiteralKind::Special(SpecialLiteralKind::Space), 
        c: ' ' 
    };
    formatter.wtr.clear(); // Clear previous output
    formatter.fmt_literal(&space_ast).unwrap();
    assert_eq!(formatter.wtr, r"\ ");
}

