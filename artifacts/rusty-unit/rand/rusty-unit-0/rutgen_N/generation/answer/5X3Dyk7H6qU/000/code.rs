// Answer 0

#[derive(Debug)]
struct BlockRng {
    core: u32,
    results: Vec<u8>,
    index: usize,
}

impl BlockRng {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("BlockRng")
            .field("core", &self.core)
            .field("result_len", &self.results.as_ref().len())
            .field("index", &self.index)
            .finish()
    }
}

#[test]
fn test_block_rng_fmt() {
    let rng = BlockRng {
        core: 42,
        results: vec![1, 2, 3, 4, 5],
        index: 3,
    };
    
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        rng.fmt(&mut formatter).unwrap();
    }

    assert!(output.contains("BlockRng"));
    assert!(output.contains("core: 42"));
    assert!(output.contains("result_len: 5"));
    assert!(output.contains("index: 3"));
}

