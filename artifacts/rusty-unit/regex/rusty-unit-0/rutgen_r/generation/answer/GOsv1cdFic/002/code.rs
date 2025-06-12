// Answer 0

#[test]
fn test_forward_many_successful_match() {
    struct MockProgram {
        matches: Vec<usize>,
    }
    
    struct MockProgramCache {
        inner: Vec<usize>,
    }
    
    let prog = MockProgram {
        matches: vec![0],
    };
    
    let mut cache = MockProgramCache {
        inner: vec![0],
    };
    
    let mut matches = vec![false];
    let text = b"example text";
    let at = 0;

    let result = forward_many(&prog, &cache, &mut matches, text, at);
    assert!(result.is_ok());
    assert_eq!(matches, vec![true]);
}

#[test]
fn test_forward_many_no_match() {
    struct MockProgram {
        matches: Vec<usize>,
    }
    
    struct MockProgramCache {
        inner: Vec<usize>,
    }
    
    let prog = MockProgram {
        matches: vec![0],
    };
    
    let mut cache = MockProgramCache {
        inner: vec![0],
    };
    
    let mut matches = vec![false];
    let text = b"no match here";
    let at = 0;

    let result = forward_many(&prog, &cache, &mut matches, text, at);
    assert!(result.is_err());
}

#[test]
fn test_forward_many_multiple_matches() {
    struct MockProgram {
        matches: Vec<usize>,
    }
    
    struct MockProgramCache {
        inner: Vec<usize>,
    }
    
    let prog = MockProgram {
        matches: vec![0, 1],
    };
    
    let mut cache = MockProgramCache {
        inner: vec![0],
    };
    
    let mut matches = vec![false, false];
    let text = b"example text that matches";
    let at = 0;

    let result = forward_many(&prog, &cache, &mut matches, text, at);
    assert!(result.is_ok());
    assert_eq!(matches, vec![true, false]); // Match found for slot 0 only
}

