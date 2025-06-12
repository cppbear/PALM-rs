// Answer 0

#[derive(Debug)]
struct TestRng;

struct DiceRoller<R> {
    rng: R,
    n: u32,
    chunk: u32,
    chunk_remaining: usize,
}

impl<R> DiceRoller<R> {
    pub fn new(rng: R, n: u32) -> Self {
        let chunk_remaining = if n == 0 { 1 } else { 0 };
        Self {
            rng,
            n,
            chunk: 0,
            chunk_remaining,
        }
    }
}

#[test]
fn test_dice_roller_non_zero_n() {
    let rng = TestRng;
    let n = 5;
    
    let roller = DiceRoller::new(rng, n);
    
    assert_eq!(roller.n, n);
    assert_eq!(roller.chunk, 0);
    assert_eq!(roller.chunk_remaining, 0);
}

#[test]
fn test_dice_roller_large_n() {
    let rng = TestRng;
    let n = 10000;
    
    let roller = DiceRoller::new(rng, n);
    
    assert_eq!(roller.n, n);
    assert_eq!(roller.chunk, 0);
    assert_eq!(roller.chunk_remaining, 0);
}

