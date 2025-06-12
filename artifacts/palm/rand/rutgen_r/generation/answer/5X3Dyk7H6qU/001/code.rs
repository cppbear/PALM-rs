// Answer 0

#[derive(Debug)]
struct Core {
    state: u32,
}

#[derive(Debug)]
struct BlockRng {
    core: Core,
    results: Vec<u8>,
    index: usize,
}

#[test]
fn test_fmt_with_empty_results() {
    let rng = BlockRng {
        core: Core { state: 0 },
        results: vec![],
        index: 0,
    };
    let mut buffer = String::new();
    let result = write!(buffer, "{:?}", rng);
    assert!(result.is_ok());
    assert_eq!(buffer, "BlockRng { core: Core { state: 0 }, result_len: 0, index: 0 }");
}

#[test]
fn test_fmt_with_non_empty_results() {
    let rng = BlockRng {
        core: Core { state: 1 },
        results: vec![1, 2, 3],
        index: 1,
    };
    let mut buffer = String::new();
    let result = write!(buffer, "{:?}", rng);
    assert!(result.is_ok());
    assert_eq!(buffer, "BlockRng { core: Core { state: 1 }, result_len: 3, index: 1 }");
}

#[test]
fn test_fmt_with_large_results() {
    let rng = BlockRng {
        core: Core { state: 2 },
        results: vec![0; 1000],
        index: 999,
    };
    let mut buffer = String::new();
    let result = write!(buffer, "{:?}", rng);
    assert!(result.is_ok());
    assert_eq!(buffer, "BlockRng { core: Core { state: 2 }, result_len: 1000, index: 999 }");
}

