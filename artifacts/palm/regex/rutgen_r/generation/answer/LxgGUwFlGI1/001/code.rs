// Answer 0

#[test]
fn test_next_pos_normal() {
    struct Input {
        pos: usize,
        len: usize,
    }
    
    let input = Input { pos: 5, len: 3 };
    assert_eq!(input.next_pos(), 8);
}

#[test]
fn test_next_pos_zero_length() {
    struct Input {
        pos: usize,
        len: usize,
    }
    
    let input = Input { pos: 0, len: 0 };
    assert_eq!(input.next_pos(), 0);
}

#[test]
fn test_next_pos_large_values() {
    struct Input {
        pos: usize,
        len: usize,
    }
    
    let input = Input { pos: usize::MAX - 1, len: 1 };
    assert_eq!(input.next_pos(), usize::MAX);
}

#[test]
fn test_next_pos_overflow() {
    struct Input {
        pos: usize,
        len: usize,
    }
    
    let input = Input { pos: usize::MAX, len: 1 };
    let result = std::panic::catch_unwind(|| {
        input.next_pos()
    });
    assert!(result.is_err());
}

