// Answer 0

#[test]
fn test_literal_searcher_iter_teddy_avx2() {
    // Helper structs
    #[derive(Clone, Debug)]
    struct DummyLiterals;

    impl DummyLiterals {
        fn empty() -> Self {
            DummyLiterals
        }
        fn all_complete(&self) -> bool {
            true
        }
        fn longest_common_prefix(&self) -> &[u8] {
            &[]
        }
        fn longest_common_suffix(&self) -> &[u8] {
            &[]
        }
    }

    #[derive(Clone, Debug)]
    struct DummyFullAcAutomaton;

    impl DummyFullAcAutomaton {
        fn patterns(&self) -> &[Vec<u8>] {
            &vec![b"pattern1".to_vec(), b"pattern2".to_vec()]
        }
    }

    let literals = DummyLiterals::empty();
    let matcher = Matcher::TeddyAVX2(Teddy {
        vb: DummyAVX2VectorBuilder,
        pats: vec![b"pattern1".to_vec(), b"pattern2".to_vec()],
        ac: DummyFullAcAutomaton,
        buckets: Vec::new(),
        masks: Masks::default(),
    });

    let searcher = LiteralSearcher::new(literals, matcher);
    match searcher.iter() {
        LiteralIter::TeddyAVX2(patterns) => {
            assert_eq!(patterns.len(), 2);
            assert_eq!(patterns[0], b"pattern1");
            assert_eq!(patterns[1], b"pattern2");
        },
        _ => panic!("Expected TeddyAVX2 but got a different variant"),
    }
}

