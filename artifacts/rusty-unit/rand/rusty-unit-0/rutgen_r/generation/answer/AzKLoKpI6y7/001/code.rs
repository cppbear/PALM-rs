// Answer 0

fn partial_shuffle_test() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rand::Rng;
    
    struct TestSlice {
        data: Vec<i32>,
    }

    impl TestSlice {
        fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        fn partial_shuffle<R>(&mut self, rng: &mut R, amount: usize) -> (&mut [i32], &mut [i32])
        where
            R: Rng + ?Sized,
        {
            let m = self.data.len().saturating_sub(amount);
            if self.data.len() < (u32::MAX as usize) {
                let mut chooser = rand::prelude::IncreasingUniform::new(rng, m as u32);
                for i in m..self.data.len() {
                    let index = chooser.next_index();
                    self.data.swap(i, index);
                }
            } else {
                for i in m..self.data.len() {
                    let index = rng.gen_range(0..i + 1);
                    self.data.swap(i, index);
                }
            }
            let r = self.data.split_at_mut(m);
            (r.1, r.0)
        }
    }

    let mut rng = StdRng::seed_from_u64(42);
    let mut test_slice = TestSlice::new(vec![1, 2, 3, 4, 5]);

    // Test case where amount is less than the length of the slice
    let (tail, head) = test_slice.partial_shuffle(&mut rng, 3);
    assert_eq!(tail.len(), 3);
    assert_eq!(head.len(), 2);

    // Test case where amount is zero
    let (tail_zero, head_zero) = test_slice.partial_shuffle(&mut rng, 0);
    assert_eq!(tail_zero.len(), 0);
    assert_eq!(head_zero.len(), 5);

    // Test case where amount equals the length of the slice
    let (tail_full, head_full) = test_slice.partial_shuffle(&mut rng, 5);
    assert_eq!(tail_full.len(), 5);
    assert_eq!(head_full.len(), 0);
}

