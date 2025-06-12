pub fn new(core: R) -> BlockRng<R> {
        let results_empty = R::Results::default();
        BlockRng {
            core,
            index: results_empty.as_ref().len(),
            results: results_empty,
        }
    }