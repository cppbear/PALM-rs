// Answer 0

#[derive(Debug)]
struct Constants {
    bit_size: usize,
    max_size_bytes: usize,
}

#[test]
fn test_should_exec_below_threshold() {
    let constants = Constants {
        bit_size: 128,
        max_size_bytes: 1024,
    };
    assert!(should_exec(5, 100));
}

#[test]
fn test_should_exec_exact_threshold() {
    let constants = Constants {
        bit_size: 128,
        max_size_bytes: 1024,
    };
    assert!(should_exec(5, 99));
}

#[test]
fn test_should_exec_above_threshold() {
    let constants = Constants {
        bit_size: 128,
        max_size_bytes: 1024,
    };
    assert!(!should_exec(5, 101));
}

#[test]
fn test_should_exec_zero_inst() {
    let constants = Constants {
        bit_size: 128,
        max_size_bytes: 1024,
    };
    assert!(should_exec(0, 100));
}

#[test]
fn test_should_exec_zero_text_len() {
    let constants = Constants {
        bit_size: 128,
        max_size_bytes: 1024,
    };
    assert!(should_exec(5, 0));
}

