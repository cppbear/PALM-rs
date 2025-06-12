// Answer 0

#[test]
fn test_iter_teddy_avx2() {
    let patterns = vec![
        vec![b'a', b'b', b'c'],
        vec![b'd', b'e', b'f'],
        vec![b'g', b'h', b'i'],
    ];
    
    let ted = TeddyAVX2 {
        vb: AVX2VectorBuilder::new(),
        pats: patterns.clone(),
        ac: FullAcAutomaton::new(patterns.clone()),
        buckets: vec![vec![0], vec![1], vec![2]],
        masks: Masks::default(),
    };

    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![b'a', b'b']),
        lcs: FreqyPacked::new(vec![b'c', b'd']),
        matcher: Matcher::TeddyAVX2(ted.clone()),
    };

    let _ = searcher.iter();
}

#[test]
fn test_iter_teddy_avx2_with_max_patterns() {
    let patterns = (0..256).map(|i| vec![i as u8]).collect::<Vec<_>>();
    
    let ted = TeddyAVX2 {
        vb: AVX2VectorBuilder::new(),
        pats: patterns.clone(),
        ac: FullAcAutomaton::new(patterns.clone()),
        buckets: vec![vec![0; 256]; 8],
        masks: Masks::default(),
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![b'a']),
        lcs: FreqyPacked::new(vec![b'b']),
        matcher: Matcher::TeddyAVX2(ted.clone()),
    };

    let _ = searcher.iter();
}

#[test]
fn test_iter_teddy_avx2_single_byte_patterns() {
    let patterns = vec![vec![b'x'], vec![b'y'], vec![b'z']];
    
    let ted = TeddyAVX2 {
        vb: AVX2VectorBuilder::new(),
        pats: patterns.clone(),
        ac: FullAcAutomaton::new(patterns.clone()),
        buckets: vec![vec![0], vec![1], vec![2]],
        masks: Masks::default(),
    };

    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![b'x']),
        lcs: FreqyPacked::new(vec![b'z']),
        matcher: Matcher::TeddyAVX2(ted.clone()),
    };

    let _ = searcher.iter();
}

#[test]
fn test_iter_teddy_avx2_edge_case_empty_patterns() {
    let patterns: Vec<Vec<u8>> = vec![];
    
    let ted = TeddyAVX2 {
        vb: AVX2VectorBuilder::new(),
        pats: patterns.clone(),
        ac: FullAcAutomaton::new(patterns.clone()),
        buckets: vec![],
        masks: Masks::default(),
    };

    let searcher = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]),
        lcs: FreqyPacked::new(vec![]),
        matcher: Matcher::TeddyAVX2(ted.clone()),
    };

    let _ = searcher.iter();
}

#[test]
fn test_iter_teddy_avx2_max_length_pattern() {
    let patterns = vec![vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p']];
    
    let ted = TeddyAVX2 {
        vb: AVX2VectorBuilder::new(),
        pats: patterns.clone(),
        ac: FullAcAutomaton::new(patterns.clone()),
        buckets: vec![vec![0]],
        masks: Masks::default(),
    };

    let searcher = LiteralSearcher {
        complete: false,
        lcp: FreqyPacked::new(vec![b'p']),
        lcs: FreqyPacked::new(vec![b'a']),
        matcher: Matcher::TeddyAVX2(ted.clone()),
    };

    let _ = searcher.iter();
}

