// Answer 0

#[test]
fn test_as_str_valid_utf8() {
    struct Alphabet {
        symbols: Vec<u8>,
    }
    
    let alphabet = Alphabet {
        symbols: vec![104, 101, 108, 108, 111], // represents "hello"
    };
    
    assert_eq!(alphabet.as_str(), "hello");
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    struct Alphabet {
        symbols: Vec<u8>,
    }
    
    let alphabet = Alphabet {
        symbols: vec![255, 254, 253], // invalid UTF-8 sequence
    };
    
    // This will panic due to invalid UTF-8
    let _ = alphabet.as_str();
} 

#[test]
fn test_as_str_empty() {
    struct Alphabet {
        symbols: Vec<u8>,
    }
    
    let alphabet = Alphabet {
        symbols: vec![], // empty symbols
    };
    
    assert_eq!(alphabet.as_str(), "");
}

#[test]
fn test_as_str_valid_ascii() {
    struct Alphabet {
        symbols: Vec<u8>,
    }
    
    let alphabet = Alphabet {
        symbols: vec![65, 66, 67], // represents "ABC"
    };
    
    assert_eq!(alphabet.as_str(), "ABC");
}

#[test]
fn test_as_str_non_ascii() {
    struct Alphabet {
        symbols: Vec<u8>,
    }
    
    let alphabet = Alphabet {
        symbols: vec![228, 189, 160, 229, 165, 189], // represents "中文"
    };
    
    assert_eq!(alphabet.as_str(), "中文");
}

