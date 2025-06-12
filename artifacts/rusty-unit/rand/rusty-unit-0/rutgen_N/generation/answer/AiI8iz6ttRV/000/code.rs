// Answer 0

#[derive(Default)]
struct TestRng;

struct CoinFlipper<R> {
    rng: R,
    chunk: u32,
    chunk_remaining: u32,
}

impl<R> CoinFlipper<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            chunk: 0,
            chunk_remaining: 0,
        }
    }
}

#[test]
fn test_coin_flipper_new() {
    let rng = TestRng::default();
    let coin_flipper = CoinFlipper::new(rng);
    assert_eq!(coin_flipper.chunk, 0);
    assert_eq!(coin_flipper.chunk_remaining, 0);
}

