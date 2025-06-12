// Answer 0

#[test]
fn test_splitn_with_valid_input() {
    let re = Regex::new(r"\W+").unwrap();
    let text = b"Hey! How are you?";
    let result: Vec<&[u8]> = re.splitn(text, 3).splits.finder.collect();
    assert_eq!(result, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);
}

#[test]
fn test_splitn_with_zero_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let text = b"Hey! How are you?";
    let result: Vec<&[u8]> = re.splitn(text, 0).splits.finder.collect();
    assert!(result.is_empty());
}

#[test]
fn test_splitn_with_one_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let text = b"Hey! How are you?";
    let result: Vec<&[u8]> = re.splitn(text, 1).splits.finder.collect();
    assert_eq!(result, vec![&b"Hey! How are you?"[..]]);
}

#[test]
fn test_splitn_with_exceeding_limit() {
    let re = Regex::new(r"\W+").unwrap();
    let text = b"Hey! How are you?";
    let result: Vec<&[u8]> = re.splitn(text, 10).splits.finder.collect();
    assert_eq!(result, vec![&b"Hey"[..], &b"How"[..], &b"are"[..], &b"you?"[..]]);
}

#[test]
fn test_splitn_with_empty_string() {
    let re = Regex::new(r"\W+").unwrap();
    let text = b"";
    let result: Vec<&[u8]> = re.splitn(text, 3).splits.finder.collect();
    assert!(result.is_empty());
}

