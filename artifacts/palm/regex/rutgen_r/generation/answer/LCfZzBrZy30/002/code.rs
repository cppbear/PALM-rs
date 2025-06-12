// Answer 0

#[derive(Debug)]
struct Item {
    value: char,
}

#[derive(Debug)]
enum ClassSet<'a> {
    Item(&'a Item),
    BinaryOp(&'a str),
}

#[derive(Debug)]
enum ClassInduct<'a> {
    Item(&'a Item),
    BinaryOp(&'a str),
}

fn from_set<'a>(ast: &'a ClassSet<'a>) -> ClassInduct<'a> {
    match *ast {
        ClassSet::Item(ref item) => ClassInduct::Item(item),
        ClassSet::BinaryOp(ref op) => ClassInduct::BinaryOp(op),
    }
}

#[test]
fn test_from_set_item() {
    let item = Item { value: 'a' };
    let ast = ClassSet::Item(&item);
    if let ClassInduct::Item(ref returned_item) = from_set(&ast) {
        assert_eq!(returned_item.value, item.value);
    } else {
        panic!("Expected ClassInduct::Item");
    }
}

#[test]
fn test_from_set_binary_op_should_panic() {
    let item = Item { value: 'a' };
    let ast = ClassSet::BinaryOp("OR");
    let result = std::panic::catch_unwind(|| {
        from_set(&ast);
    });

    assert!(result.is_err());
}

