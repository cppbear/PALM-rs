// Answer 0

#[test]
fn test_pop_class_with_non_empty_union_tail() {
    use ast::{ClassSetItem, ClassFrame};
    
    let item_a = ClassSetItem::Literal('a');
    let item_b = ClassSetItem::Literal('b');
    let tail: Vec<ClassSetItem> = vec![item_a.clone(), item_b.clone()];

    let induct = ClassFrame::Union {
        head: &item_a,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);
}

#[test]
fn test_pop_class_with_two_items_in_tail() {
    use ast::{ClassSetItem, ClassFrame};

    let item_a = ClassSetItem::Literal('a');
    let item_b = ClassSetItem::Literal('b');
    let item_c = ClassSetItem::Literal('c');
    let tail: Vec<ClassSetItem> = vec![item_b.clone(), item_c.clone()];

    let induct = ClassFrame::Union {
        head: &item_a,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);
}

#[test]
fn test_pop_class_with_large_tail() {
    use ast::{ClassSetItem, ClassFrame};

    let item_a = ClassSetItem::Literal('a');
    let tail: Vec<ClassSetItem> = (b'b'..=b'z').map(|b| ClassSetItem::Literal(b as char)).collect();

    let induct = ClassFrame::Union {
        head: &item_a,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(induct);
}

