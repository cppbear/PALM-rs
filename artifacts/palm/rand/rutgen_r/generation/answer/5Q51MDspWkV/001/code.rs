// Answer 0

#[test]
fn test_fmt() {
    struct Core {}
    
    struct BlockRng64 {
        core: Core,
        results: Vec<u8>,
        index: usize,
        half_used: bool,
    }

    impl BlockRng64 {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("BlockRng64")
                .field("core", &self.core)
                .field("result_len", &self.results.as_ref().len())
                .field("index", &self.index)
                .field("half_used", &self.half_used)
                .finish()
        }
    }

    let core = Core {};
    let results = vec![1, 2, 3, 4, 5]; // Non-empty vector for testing
    let index = 0; // Starting index
    let half_used = false; // Testing case where half_used is false
    let block_rng = BlockRng64 { core, results, index, half_used };

    let mut output = String::new();
    let result = block_rng.fmt(&mut std::fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert!(output.contains("BlockRng64"));
    assert!(output.contains("result_len"));
    assert!(output.contains("index"));
    assert!(output.contains("half_used"));
}

#[test]
fn test_fmt_with_empty_results() {
    struct Core {}
    
    struct BlockRng64 {
        core: Core,
        results: Vec<u8>,
        index: usize,
        half_used: bool,
    }

    impl BlockRng64 {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("BlockRng64")
                .field("core", &self.core)
                .field("result_len", &self.results.as_ref().len())
                .field("index", &self.index)
                .field("half_used", &self.half_used)
                .finish()
        }
    }

    let core = Core {};
    let results: Vec<u8> = vec![]; // Testing with an empty vector
    let index = 0; 
    let half_used = false; 
    let block_rng = BlockRng64 { core, results, index, half_used };

    let mut output = String::new();
    let result = block_rng.fmt(&mut std::fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert!(output.contains("BlockRng64"));
    assert!(output.contains("result_len"));
    assert!(output.contains("index"));
    assert!(output.contains("half_used"));
}

