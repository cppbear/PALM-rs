// Answer 0

#[test]
fn test_invalid_last_symbol_0() {
    let error = DecodeError::InvalidLastSymbol(0, 0);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_invalid_last_symbol_1() {
    let error = DecodeError::InvalidLastSymbol(1, 1);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_invalid_last_symbol_2() {
    let error = DecodeError::InvalidLastSymbol(2, 255);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_invalid_last_symbol_3() {
    let error = DecodeError::InvalidLastSymbol(3, 128);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_invalid_last_symbol_edge_cases() {
    let error = DecodeError::InvalidLastSymbol(3, 0);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
    
    let error = DecodeError::InvalidLastSymbol(0, 255);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

