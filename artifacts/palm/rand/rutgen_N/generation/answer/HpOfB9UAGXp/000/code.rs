// Answer 0

#[derive(Debug)]
struct R;

impl R {
    fn from_seed(seed: [u8; 32]) -> Self {
        R
    }
}

struct MyRandBlock;

impl MyRandBlock {
    type Seed = [u8; 32];

    fn new(r: R) -> Self {
        MyRandBlock
    }

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
    }
}

#[test]
fn test_from_seed_valid() {
    let seed: [u8; 32] = [0; 32];
    let rand_block = MyRandBlock::from_seed(seed);
    assert_eq!(format!("{:?}", rand_block), format!("{:?}", MyRandBlock));
}

#[test]
#[should_panic]
fn test_from_seed_invalid() {
    let seed: [u8; 32] = [1; 31]; // This is an invalid seed length.
    let _ = MyRandBlock::from_seed(seed);
}

