// Answer 0

#[test]
fn test_class_binary_a() {
    let query = ClassQuery::Binary("A");
    class(query);
}

#[test]
fn test_class_binary_z() {
    let query = ClassQuery::Binary("Z");
    class(query);
}

#[test]
fn test_class_script_latin() {
    let query = ClassQuery::Script("Latin");
    class(query);
}

#[test]
fn test_class_script_cyrillic() {
    let query = ClassQuery::Script("Cyrillic");
    class(query);
}

#[test]
fn test_class_script_han() {
    let query = ClassQuery::Script("Han");
    class(query);
}

