// Answer 0

fn test_compile_valid_case() {
    struct C {
        utf8_seqs: Option<Utf8Seqs>,
        suffix_cache: Vec<usize>,
        insts: Vec<usize>,
    }

    impl C {
        fn fill(&mut self, _last_split: Hole, _entry: usize) {}
        fn fill_to_next(&mut self, _last_split: Hole) {}
        fn push_split_hole(&mut self) -> Hole {
            Hole::None
        }
        fn fill_split(&mut self, _last_split: Hole, _entry: Option<usize>, _arg: Option<usize>) -> Hole {
            Hole::None
        }
    }

    struct Utf8Seqs;

    impl Utf8Seqs {
        fn take(&mut self) -> Option<Utf8Seqs> {
            Some(Utf8Seqs)
        }
        fn reset(&mut self, _start: usize, _end: usize) {}
    }

    struct Compile {
        c: C,
        c_utf8_seq: fn(&Utf8Seq) -> Result<Patch>,
        ranges: Vec<std::ops::Range<usize>>,
    }

    struct Utf8Seq;

    struct Patch {
        hole: Hole,
        entry: usize,
    }

    enum Hole {
        None,
        Many(Vec<Hole>),
    }

    let c = C {
        utf8_seqs: Some(Utf8Seqs),
        suffix_cache: vec![],
        insts: vec![],
    };

    let c_utf8_seq = |_: &Utf8Seq| -> Result<Patch> {
        Ok(Patch {
            hole: Hole::None,
            entry: 0,
        })
    };

    let ranges = vec![0..5, 5..10];

    let mut compile = Compile {
        c,
        c_utf8_seq,
        ranges,
    };

    let result = compile.compile();

    assert!(result.is_ok());
    let patch = result.unwrap();
    match patch.hole {
        Hole::Many(ref holes) => assert!(!holes.is_empty()),
        _ => panic!("Expected many holes"),
    }
    assert_eq!(patch.entry, 0);
}

fn test_compile_empty_ranges() {
    struct C {
        utf8_seqs: Option<Utf8Seqs>,
        suffix_cache: Vec<usize>,
        insts: Vec<usize>,
    }

    impl C {
        fn fill(&mut self, _last_split: Hole, _entry: usize) {}
        fn fill_to_next(&mut self, _last_split: Hole) {}
        fn push_split_hole(&mut self) -> Hole {
            Hole::None
        }
        fn fill_split(&mut self, _last_split: Hole, _entry: Option<usize>, _arg: Option<usize>) -> Hole {
            Hole::None
        }
    }

    struct Utf8Seqs;

    impl Utf8Seqs {
        fn take(&mut self) -> Option<Utf8Seqs> {
            Some(Utf8Seqs)
        }
        fn reset(&mut self, _start: usize, _end: usize) {}
    }

    struct Compile {
        c: C,
        c_utf8_seq: fn(&Utf8Seq) -> Result<Patch>,
        ranges: Vec<std::ops::Range<usize>>,
    }

    struct Utf8Seq;

    struct Patch {
        hole: Hole,
        entry: usize,
    }

    enum Hole {
        None,
        Many(Vec<Hole>),
    }

    let c = C {
        utf8_seqs: Some(Utf8Seqs),
        suffix_cache: vec![],
        insts: vec![],
    };

    let c_utf8_seq = |_: &Utf8Seq| -> Result<Patch> {
        Ok(Patch {
            hole: Hole::None,
            entry: 0,
        })
    };

    let ranges: Vec<std::ops::Range<usize>> = vec![];

    let mut compile = Compile {
        c,
        c_utf8_seq,
        ranges,
    };

    let result = compile.compile();

    assert!(result.is_ok());
    let patch = result.unwrap();
    assert_eq!(patch.hole, Hole::Many(vec![]));
    assert_eq!(patch.entry, 0);
}

