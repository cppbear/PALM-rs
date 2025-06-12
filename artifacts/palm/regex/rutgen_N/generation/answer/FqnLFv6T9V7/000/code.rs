// Answer 0

#[derive(Debug)]
struct RegexWrapper(Vec<String>);

impl RegexWrapper {
    fn new(captures: Vec<String>) -> Self {
        RegexWrapper(captures)
    }

    fn len(&self) -> usize {
        self.0.len() / 2
    }
}

#[test]
fn test_len_with_two_capturing_groups() {
    let regex = RegexWrapper::new(vec![String::from("group1"), String::from("group2")]);
    assert_eq!(regex.len(), 1);
}

#[test]
fn test_len_with_no_capturing_groups() {
    let regex = RegexWrapper::new(vec![]);
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_len_with_one_capturing_group() {
    let regex = RegexWrapper::new(vec![String::from("group1")]);
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_len_with_three_capturing_groups() {
    let regex = RegexWrapper::new(vec![String::from("group1"), String::from("group2"), String::from("group3")]);
    assert_eq!(regex.len(), 1);
}

#[test]
fn test_len_with_five_capturing_groups() {
    let regex = RegexWrapper::new(vec![String::from("group1"), String::from("group2"), String::from("group3"), String::from("group4"), String::from("group5")]);
    assert_eq!(regex.len(), 2);
}

