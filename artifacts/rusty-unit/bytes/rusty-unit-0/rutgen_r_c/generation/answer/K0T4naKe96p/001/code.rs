// Answer 0

#[test]
fn test_chain_new_with_integers() {
    let a = 5;
    let b = 10;
    let chain = Chain::new(a, b);
    assert_eq!(chain.a, 5);
    assert_eq!(chain.b, 10);
}

#[test]
fn test_chain_new_with_strings() {
    let a = String::from("Hello");
    let b = String::from("World");
    let chain = Chain::new(a.clone(), b.clone());
    assert_eq!(chain.a, a);
    assert_eq!(chain.b, b);
}

#[test]
fn test_chain_new_with_floats() {
    let a = 3.14;
    let b = 1.59;
    let chain = Chain::new(a, b);
    assert_eq!(chain.a, 3.14);
    assert_eq!(chain.b, 1.59);
}

#[test]
fn test_chain_new_with_empty_string() {
    let a = String::from("");
    let b = String::from("");
    let chain = Chain::new(a.clone(), b.clone());
    assert_eq!(chain.a, a);
    assert_eq!(chain.b, b);
}

#[test]
fn test_chain_new_with_boolean() {
    let a = true;
    let b = false;
    let chain = Chain::new(a, b);
    assert_eq!(chain.a, true);
    assert_eq!(chain.b, false);
}

