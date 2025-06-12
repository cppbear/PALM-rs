// Answer 0

#[derive(Debug)]
struct StringWrapper<'a>(&'a str);

fn generic_levenshtein(a: &StringWrapper, b: &StringWrapper) -> usize {
    let (len_a, len_b) = (a.0.len(), b.0.len());
    let mut d = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        d[i][0] = i;
    }
    for j in 0..=len_b {
        d[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a.0.as_bytes()[i - 1] == b.0.as_bytes()[j - 1] { 0 } else { 1 };
            d[i][j] = *[
                d[i - 1][j] + 1,       // deletion
                d[i][j - 1] + 1,       // insertion
                d[i - 1][j - 1] + cost, // substitution
            ].iter().min().unwrap();
        }
    }

    d[len_a][len_b]
}

#[test]
fn test_levenshtein_same_strings() {
    assert_eq!(levenshtein("test", "test"), 0);
}

#[test]
fn test_levenshtein_one_insertion() {
    assert_eq!(levenshtein("test", "tests"), 1);
}

#[test]
fn test_levenshtein_one_deletion() {
    assert_eq!(levenshtein("tests", "test"), 1);
}

#[test]
fn test_levenshtein_one_substitution() {
    assert_eq!(levenshtein("test", "best"), 1);
}

#[test]
fn test_levenshtein_multiple_operations() {
    assert_eq!(levenshtein("kitten", "sitting"), 3);
}

#[test]
fn test_levenshtein_empty_strings() {
    assert_eq!(levenshtein("", ""), 0);
    assert_eq!(levenshtein("a", ""), 1);
    assert_eq!(levenshtein("", "a"), 1);
}

