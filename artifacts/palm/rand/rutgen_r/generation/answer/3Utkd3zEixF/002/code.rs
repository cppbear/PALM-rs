// Answer 0

#[derive(Debug)]
struct RngMock;

struct DiceRoller<R> {
    rng: R,
    n: u32,
    chunk: u32,
    chunk_remaining: u32,
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
fn test_dice_roller_with_zero() {
    let rng = RngMock;
    let n = 0;
    let dice_roller = DiceRoller::new(rng, n);
    assert_eq!(dice_roller.n, 0);
    assert_eq!(dice_roller.chunk, 0);
    assert_eq!(dice_roller.chunk_remaining, 1);
}

