// Answer 0

#[test]
fn test_teddy_len_empty_patterns() {
    let pats: Vec<Vec<u8>> = vec![];
    let vb = SSSE3VectorBuilder(());
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks {
        vb: vb.clone(),
        masks: [Mask::default(); 3],
        size: 0,
    };
    let teddy = Teddy {
        vb,
        pats,
        ac,
        buckets: vec![vec![]; 8],
        masks,
    };
    assert_eq!(teddy.len(), 0);
}

#[test]
fn test_teddy_len_single_pattern() {
    let pats: Vec<Vec<u8>> = vec![b"pattern1".to_vec()];
    let vb = SSSE3VectorBuilder(());
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks {
        vb: vb.clone(),
        masks: [Mask::default(); 3],
        size: 1,
    };
    let teddy = Teddy {
        vb,
        pats,
        ac,
        buckets: vec![vec![]; 8],
        masks,
    };
    assert_eq!(teddy.len(), 1);
}

#[test]
fn test_teddy_len_multiple_patterns() {
    let pats: Vec<Vec<u8>> = vec![b"pattern1".to_vec(), b"pattern2".to_vec()];
    let vb = SSSE3VectorBuilder(());
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks {
        vb: vb.clone(),
        masks: [Mask::default(); 3],
        size: 2,
    };
    let teddy = Teddy {
        vb,
        pats,
        ac,
        buckets: vec![vec![]; 8],
        masks,
    };
    assert_eq!(teddy.len(), 2);
}

