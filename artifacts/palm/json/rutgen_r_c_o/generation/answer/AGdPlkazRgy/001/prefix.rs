// Answer 0

#[test]
fn test_numeric_strings() {
    let key_classifier = KeyClassifier;
    let numerical_inputs = vec!["0", "1", "123", "-1", "3.14", "1e10"];
    
    for &input in &numerical_inputs {
        key_classifier.deserialize(input).unwrap();
    }
}

#[test]
fn test_non_numeric_strings() {
    let key_classifier = KeyClassifier;
    let non_numerical_inputs = vec!["abc", "1abc", "", " ", "NaN"];
    
    for &input in &non_numerical_inputs {
        let result = key_classifier.deserialize(input);
        assert!(result.is_err());
    }
}

#[test]
fn test_special_characters() {
    let key_classifier = KeyClassifier;
    let special_inputs = vec!["#", "&", "%", "!"];
    
    for &input in &special_inputs {
        let result = key_classifier.deserialize(input);
        assert!(result.is_err());
    }
}

#[test]
fn test_large_values() {
    let key_classifier = KeyClassifier;
    let large_inputs = vec!["999999999999999", "1e308"];
    
    for &input in &large_inputs {
        key_classifier.deserialize(input).unwrap();
    }
}

#[test]
fn test_edge_cases() {
    let key_classifier = KeyClassifier;
    let edge_cases = vec!["0.0", "-0", "1.0e-5", "Infinity"];
    
    for &input in &edge_cases {
        let result = key_classifier.deserialize(input);
        assert!(result.is_err());
    }
}

