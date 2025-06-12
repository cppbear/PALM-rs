// Answer 0

#[derive(Clone)]
struct Literals(Vec<String>);

impl Literals {
    fn unambiguous_suffixes(&self) -> Literals {
        let mut lits = self.clone();
        lits.0.reverse();
        let mut unamb = lits.unambiguous_prefixes();
        unamb.0.reverse();
        unamb
    }

    fn unambiguous_prefixes(&self) -> Literals {
        // Placeholder implementation for example purposes.
        let unique: Vec<String> = self.0.iter().map(|s| s.clone()).collect(); // Assume it eliminates ambiguities
        Literals(unique)
    }
}

#[test]
fn test_unambiguous_suffixes_empty() {
    let literals = Literals(vec![]);
    let result = literals.unambiguous_suffixes();
    assert!(result.0.is_empty());
}

#[test]
fn test_unambiguous_suffixes_single_element() {
    let literals = Literals(vec!["a".to_string()]);
    let result = literals.unambiguous_suffixes();
    assert_eq!(result.0, vec!["a".to_string()]);
}

#[test]
fn test_unambiguous_suffixes_no_overlap() {
    let literals = Literals(vec!["abc".to_string(), "def".to_string()]);
    let result = literals.unambiguous_suffixes();
    assert_eq!(result.0, vec!["abc".to_string(), "def".to_string()]);
}

#[test]
fn test_unambiguous_suffixes_with_overlap() {
    let literals = Literals(vec!["abc".to_string(), "ab".to_string()]);
    let result = literals.unambiguous_suffixes();
    assert_eq!(result.0, vec!["abc".to_string()]);
}

#[test]
fn test_unambiguous_suffixes_with_substring() {
    let literals = Literals(vec!["abc".to_string(), "a".to_string(), "b".to_string()]);
    let result = literals.unambiguous_suffixes();
    assert_eq!(result.0, vec!["abc".to_string(), "b".to_string()]);
}

