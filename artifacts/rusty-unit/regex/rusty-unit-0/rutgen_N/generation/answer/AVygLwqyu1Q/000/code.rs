// Answer 0

#[derive(Debug)]
struct Visitor {
    depth: usize,
}

impl Visitor {
    fn decrement_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    fn visit_class_set_item_post(
        &mut self,
        ast: &ClassSetItem,
    ) -> Result<()> {
        match *ast {
            ClassSetItem::Empty(_) 
            | ClassSetItem::Literal(_) 
            | ClassSetItem::Range(_) 
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

#[derive(Debug)]
enum ClassSetItem {
    Empty(std::marker::PhantomData<()>),
    Literal(std::marker::PhantomData<()>),
    Range(std::marker::PhantomData<()>),
    Ascii(std::marker::PhantomData<()>),
    Unicode(std::marker::PhantomData<()>),
    Perl(std::marker::PhantomData<()>),
    Bracketed(std::marker::PhantomData<()>),
    Union(std::marker::PhantomData<()>),
}

#[derive(Debug)]
struct Result<T> {
    value: Option<T>,
}

#[test]
fn test_visit_class_set_item_post_empty() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Empty(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_class_set_item_post_literal() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Literal(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_class_set_item_post_range() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Range(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Ascii(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Unicode(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_class_set_item_post_perl() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Perl(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_class_set_item_post_bracketed() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Bracketed(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 0);
}

#[test]
fn test_visit_class_set_item_post_union() {
    let mut visitor = Visitor { depth: 1 };
    let ast = ClassSetItem::Union(std::marker::PhantomData);
    assert!(visitor.visit_class_set_item_post(&ast).is_ok());
    assert_eq!(visitor.depth, 0);
}

