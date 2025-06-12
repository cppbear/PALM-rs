// Answer 0

#[derive(Debug, PartialEq)]
pub enum StrSimError {
    DifferentLengthArgs,
}

pub type HammingResult = Result<usize, StrSimError>;

fn generic_hamming<I1, I2>(a: I1, b: I2) -> HammingResult
where
    I1: Iterator<Item = char>,
    I2: Iterator<Item = char>,
{
    let (a_len, b_len) = (a.len(), b.len());
    if a_len != b_len {
        return Err(StrSimError::DifferentLengthArgs);
    }
    Ok(a.zip(b).filter(|(x, y)| x != y).count())
}

#[test]
fn test_hamming_equal_length_different() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_equal_length_same() {
    assert_eq!(Ok(0), hamming("hamming", "hamming"));
}

#[test]
#[should_panic]
fn test_hamming_different_length() {
    match hamming("hamming", "ham") {
        Ok(_) => panic!("Expected an error for different lengths"),
        Err(StrSimError::DifferentLengthArgs) => (),
    }
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
#[should_panic]
fn test_hamming_one_empty() {
    match hamming("hamming", "") {
        Ok(_) => panic!("Expected an error for different lengths"),
        Err(StrSimError::DifferentLengthArgs) => (),
    }
}

