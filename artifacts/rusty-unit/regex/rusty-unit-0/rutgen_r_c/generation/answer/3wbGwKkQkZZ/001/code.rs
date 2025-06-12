// Answer 0

#[test]
fn test_approximate_size_empty_patterns() {
    let vb = SSSE3VectorBuilder(());
    let pats: Vec<Vec<u8>> = Vec::new();
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks { vb, masks: [Mask {}, Mask {}, Mask {}], size: pats.len() };
    let teddy = Teddy { vb, pats, ac, buckets: Vec::new(), masks };

    assert_eq!(teddy.approximate_size(), 0);
}

#[test]
fn test_approximate_size_single_empty_pattern() {
    let vb = SSSE3VectorBuilder(());
    let pats: Vec<Vec<u8>> = vec![Vec::new()];
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks { vb, masks: [Mask {}, Mask {}, Mask {}], size: pats.len() };
    let teddy = Teddy { vb, pats, ac, buckets: Vec::new(), masks };

    assert_eq!(teddy.approximate_size(), 0);
}

#[test]
fn test_approximate_size_multiple_empty_patterns() {
    let vb = SSSE3VectorBuilder(());
    let pats: Vec<Vec<u8>> = vec![Vec::new(), Vec::new(), Vec::new()];
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks { vb, masks: [Mask {}, Mask {}, Mask {}], size: pats.len() };
    let teddy = Teddy { vb, pats, ac, buckets: Vec::new(), masks };

    assert_eq!(teddy.approximate_size(), 0);
}

#[test]
fn test_approximate_size_non_empty_patterns() {
    let vb = SSSE3VectorBuilder(());
    let pats: Vec<Vec<u8>> = vec![b"abc".to_vec(), b"def".to_vec()];
    let ac = FullAcAutomaton::new(pats.clone());
    let masks = Masks { vb, masks: [Mask {}, Mask {}, Mask {}], size: pats.len() };
    let teddy = Teddy { vb, pats, ac, buckets: Vec::new(), masks };

    assert_eq!(teddy.approximate_size(), 6);
}

