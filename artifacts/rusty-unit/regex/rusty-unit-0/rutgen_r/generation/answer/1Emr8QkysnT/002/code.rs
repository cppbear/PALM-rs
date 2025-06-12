// Answer 0

#[test]
fn test_approximate_size_teddy_ssse3() {
    struct TestMatcher {
        matcher: Matcher,
    }

    enum Matcher {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(Box<dyn ApproximateSize>),
        BoyerMoore(Box<dyn ApproximateSize>),
        AC(Box<dyn Aut>),
        TeddySSSE3(Box<Teddy>),
        TeddyAVX2(Box<Teddy>),
    }

    trait ApproximateSize {
        fn approximate_size(&self) -> usize;
    }

    struct Teddy {
        size: usize,
    }

    impl ApproximateSize for Teddy {
        fn approximate_size(&self) -> usize {
            self.size
        }
    }

    impl TestMatcher {
        pub fn approximate_size(&self) -> usize {
            use self::Matcher::*;
            match &self.matcher {
                Empty => 0,
                Bytes(sset) => sset.len(),
                FreqyPacked(single) => single.approximate_size(),
                BoyerMoore(single) => single.approximate_size(),
                AC(aut) => aut.heap_bytes(),
                TeddySSSE3(ted) => ted.approximate_size(),
                TeddyAVX2(ted) => ted.approximate_size(),
            }
        }
    }

    let teddy = Teddy { size: 128 };
    let matcher = TestMatcher {
        matcher: Matcher::TeddySSSE3(Box::new(teddy)),
    };

    assert_eq!(matcher.approximate_size(), 128);
}

