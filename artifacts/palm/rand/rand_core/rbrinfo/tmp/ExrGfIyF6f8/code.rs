pub fn new(core: R) -> BlockRng64<R> {
        let results_empty = R::Results::default();
        BlockRng64 {
            core,
            index: results_empty.as_ref().len(),
            half_used: false,
            results: results_empty,
        }
    }