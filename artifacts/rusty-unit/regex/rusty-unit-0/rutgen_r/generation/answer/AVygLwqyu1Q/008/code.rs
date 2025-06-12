// Answer 0

#[derive(Debug)]
enum ClassSetItem {
    Empty(),
    Literal(char),
    Range(char, char),
    Ascii(u8),
    Unicode(char),
    Perl(String),
    Bracketed(),
    Union(),
}

struct Visitor {
    depth: usize,
}

impl Visitor {
    fn decrement_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    fn visit_class_set_item_post(&mut self, ast: &ClassSetItem) -> Result<(), ()> {
        match *ast {
            ClassSetItem::Empty(_)
            | ClassSetItem::Literal(_)
            | ClassSetItem::Range(_, _)
            | ClassSetItem::Ascii(_)
            | ClassSetItem::Unicode(_)
            | ClassSetItem::Perl(_) => {
                // These are all base cases, so we don't decrement depth.
                Ok(())
            }
            ClassSetItem::Bracketed(_)
            | ClassSetItem::Union(_) => {
                self.decrement_depth();
                Ok(())
            }
        }
    }
}

#[test]
fn test_visit_class_set_item_post_empty() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Empty();
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_literal() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Literal('a');
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_range() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Range('a', 'z');
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Ascii(97);
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Unicode('ü');
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_class_set_item_post_perl() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Perl(String::from("\\d"));
    let result = visitor.visit_class_set_item_post(&ast);
    assert_eq!(result, Ok(()));
}

