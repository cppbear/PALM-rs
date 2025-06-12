// Answer 0

#[test]
fn test_advance_a_rem_not_zero_and_a_rem_less_than_cnt() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain<'a> {
        a: Buffer,
        b: Buffer,
    }

    // Test with `a` having remaining bytes and `cnt` greater than `a_rem`
    let mut chain = Chain {
        a: Buffer { data: vec![1, 2, 3], position: 0 },
        b: Buffer { data: vec![4, 5, 6, 7], position: 0 },
    };

    // a_rem is 3, and we will advance using a cnt that is greater than 3
    let cnt = 5;

    // Test that advancing works as expected
    chain.advance(cnt);

    // `a` should be fully advanced (position should now be at 3)
    assert_eq!(chain.a.position, 3);
    // `b` should have advanced by the remaining 2
    assert_eq!(chain.b.position, 2);
}

#[test]
#[should_panic]
fn test_advance_panic_case_a_rem_zero() {
    struct Buffer {
        data: Vec<u8>,
        position: usize,
    }

    impl Buffer {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    struct Chain<'a> {
        a: Buffer,
        b: Buffer,
    }

    let mut chain = Chain {
        a: Buffer { data: vec![], position: 0 }, // Empty buffer should result in a_rem = 0
        b: Buffer { data: vec![4, 5, 6, 7], position: 0 },
    };

    // Since `a` has no remaining bytes, this should panic when trying to call advance
    let cnt = 3;
    chain.advance(cnt);
}

