pub fn simple_fold(c: char) -> result::Result<SimpleFoldIter, Option<char>> {
    CASE_FOLDING_SIMPLE
        .binary_search_by_key(&c, |&(c1, _)| c1)
        .map(|i| SimpleFoldIter(CASE_FOLDING_SIMPLE[i].1.iter()))
        .map_err(|i| {
            if i >= CASE_FOLDING_SIMPLE.len() {
                None
            } else {
                Some(CASE_FOLDING_SIMPLE[i].0)
            }
        })
}