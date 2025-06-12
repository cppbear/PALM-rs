// Answer 0

#[derive(Debug)]
enum ClassSetItem {
    Empty(),
    Literal(char),
    Range(char, char),
    Ascii(u8),
    Perl(u8),
    Unicode(u32),
    Bracketed(),
    Union(),
}

#[derive(Debug)]
struct ClassInduct {
    item: ClassSetItem,
}

impl std::fmt::Display for ClassInduct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = match &self.item {
            ClassSetItem::Empty() => "Item(Empty)",
            ClassSetItem::Literal(_) => "Item(Literal)",
            ClassSetItem::Range(_, _) => "Item(Range)",
            ClassSetItem::Ascii(_) => "Item(Ascii)",
            ClassSetItem::Perl(_) => "Item(Perl)",
            ClassSetItem::Unicode(_) => "Item(Unicode)",
            ClassSetItem::Bracketed() => "Item(Bracketed)",
            ClassSetItem::Union() => "Item(Union)",
        };
        write!(f, "{}", x)
    }
}

#[test]
fn test_class_induct_item_empty() {
    let item = ClassSetItem::Empty();
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Empty)");
}

#[test]
fn test_class_induct_item_literal() {
    let item = ClassSetItem::Literal('a');
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Literal)");
}

#[test]
fn test_class_induct_item_range() {
    let item = ClassSetItem::Range('a', 'z');
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Range)");
}

#[test]
fn test_class_induct_item_ascii() {
    let item = ClassSetItem::Ascii(97); // ASCII for 'a'
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Ascii)");
}

#[test]
fn test_class_induct_item_perl() {
    let item = ClassSetItem::Perl(0);
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Perl)");
}

#[test]
fn test_class_induct_item_unicode() {
    let item = ClassSetItem::Unicode(0x0041); // Unicode for 'A'
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Unicode)");
}

#[test]
fn test_class_induct_item_bracketed() {
    let item = ClassSetItem::Bracketed();
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Bracketed)");
}

#[test]
fn test_class_induct_item_union() {
    let item = ClassSetItem::Union();
    let class_induct = ClassInduct { item };
    let mut output = String::new();
    let result = write!(&mut output, "{}", class_induct);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Union)");
}

