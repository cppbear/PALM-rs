// Answer 0

#[test]
fn test_deref_valid_case() {
    struct Sparse {
        dense: Vec<i32>,
        size: usize,
    }

    impl Sparse {
        fn deref(&self) -> &[i32] {
            &self.dense[0..self.size]
        }
    }

    let sparse = Sparse {
        dense: vec![1, 2, 3, 4, 5],
        size: 3,
    };

    let result = sparse.deref();
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deref_size_zero() {
    struct Sparse {
        dense: Vec<i32>,
        size: usize,
    }

    impl Sparse {
        fn deref(&self) -> &[i32] {
            &self.dense[0..self.size]
        }
    }

    let sparse = Sparse {
        dense: vec![], // empty vec
        size: 0,
    };

    let result = sparse.deref();
    assert_eq!(result, &[]);
}

#[test]
#[should_panic]
fn test_deref_exceeding_size() {
    struct Sparse {
        dense: Vec<i32>,
        size: usize,
    }

    impl Sparse {
        fn deref(&self) -> &[i32] {
            &self.dense[0..self.size]
        }
    }

    let sparse = Sparse {
        dense: vec![1, 2, 3],
        size: 4, // size exceeds the length of dense
    };

    let _result = sparse.deref();
}

