// Answer 0

#[derive(Debug, Clone, PartialEq, Eq)]
struct Literals {
    lits: Vec<String>,
}

impl Literals {
    pub fn min_len(&self) -> Option<usize> {
        self.lits.iter().map(|lit| lit.len()).min()
    }

    pub fn to_empty(&self) -> Literals {
        Literals { lits: Vec::new() }
    }

    pub fn trim_suffix(&self, num_bytes: usize) -> Option<Literals> {
        if self.min_len().map(|len| len <= num_bytes).unwrap_or(true) {
            return None;
        }
        let mut new = self.to_empty();
        for mut lit in self.lits.iter().cloned() {
            let new_len = lit.len().checked_sub(num_bytes).unwrap_or(0);
            lit.truncate(new_len);
            new.lits.push(lit);
        }
        new.lits.sort();
        new.lits.dedup();
        Some(new)
    }
}

#[test]
fn test_trim_suffix_some() {
    let literals = Literals {
        lits: vec!["abcde".to_string(), "fghij".to_string(), "klmno".to_string()],
    };
    let result = literals.trim_suffix(2);
    let expected = Some(Literals {
        lits: vec!["abc".to_string(), "fgh".to_string(), "klm".to_string()],
    });
    assert_eq!(result, expected);
}

#[test]
fn test_trim_suffix_none() {
    let literals = Literals {
        lits: vec!["abc".to_string()],
    };
    let result = literals.trim_suffix(5);
    assert_eq!(result, None);
}

#[test]
fn test_trim_suffix_duplicates() {
    let literals = Literals {
        lits: vec![
            "abcde".to_string(),
            "abcde".to_string(),
            "fghij".to_string(),
        ],
    };
    let result = literals.trim_suffix(2);
    let expected = Some(Literals {
        lits: vec!["abc".to_string(), "fgh".to_string()],
    });
    assert_eq!(result, expected);
}

#[test]
fn test_trim_suffix_zero_bytes() {
    let literals = Literals {
        lits: vec!["test".to_string(), "test2".to_string()],
    };
    let result = literals.trim_suffix(0);
    let expected = Some(Literals {
        lits: vec!["test".to_string(), "test2".to_string()],
    });
    assert_eq!(result, expected);
}

