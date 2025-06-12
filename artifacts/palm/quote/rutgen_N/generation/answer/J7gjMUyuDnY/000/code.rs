// Answer 0

#[test]
fn test_append_separated_empty_iterator() {
    let mut tokens = vec![];
    let iter: Vec<&str> = vec![];
    let separator = "separator";

    append_separated(&mut tokens, iter.iter(), separator);

    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_append_separated_single_item() {
    let mut tokens = vec![];
    let iter: Vec<&str> = vec!["item1"];
    let separator = "separator";

    append_separated(&mut tokens, iter.iter(), separator);

    assert_eq!(tokens, vec!["item1"]);
}

#[test]
fn test_append_separated_multiple_items() {
    let mut tokens = vec![];
    let iter: Vec<&str> = vec!["item1", "item2", "item3"];
    let separator = "separator";

    append_separated(&mut tokens, iter.iter(), separator);

    assert_eq!(tokens, vec!["item1", "separator", "item2", "separator", "item3"]);
}

#[test]
fn test_append_separated_with_different_types() {
    let mut tokens = vec![];
    let iter: Vec<String> = vec![String::from("item1"), String::from("item2")];
    let separator = "separator";

    append_separated(&mut tokens, iter.iter(), separator);

    assert_eq!(tokens, vec!["item1", "separator", "item2"]);
}

