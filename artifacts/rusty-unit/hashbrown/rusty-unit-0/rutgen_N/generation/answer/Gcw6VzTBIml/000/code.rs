// Answer 0

#[derive(Debug)]
struct NonZeroBitMaskWord(u64);

impl NonZeroBitMaskWord {
    fn new_unchecked(value: u64) -> Self {
        assert!(value != 0);
        NonZeroBitMaskWord(value)
    }

    fn get(&self) -> u64 {
        self.0
    }

    fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }
    
    fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }
}

const BITMASK_STRIDE: usize = 8;

fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {
    if cfg!(target_arch = "arm") && BITMASK_STRIDE % 8 == 0 {
        let swapped = unsafe { NonZeroBitMaskWord::new_unchecked(nonzero.get().swap_bytes()) };
        swapped.leading_zeros() as usize / BITMASK_STRIDE
    } else {
        nonzero.trailing_zeros() as usize / BITMASK_STRIDE
    }
}

#[test]
fn test_nonzero_trailing_zeros() {
    let nonzero_value = NonZeroBitMaskWord::new_unchecked(0b0000000000000000000000000000000000000000000000000000000000001110);
    assert_eq!(nonzero_trailing_zeros(nonzero_value), 0);

    let nonzero_value = NonZeroBitMaskWord::new_unchecked(0b0000000000000000000000000000000000000000000000000000000000011100);
    assert_eq!(nonzero_trailing_zeros(nonzero_value), 2);
    
    let nonzero_value = NonZeroBitMaskWord::new_unchecked(0b0000000000000000000000000000000000000000000000000000000000111111);
    assert_eq!(nonzero_trailing_zeros(nonzero_value), 0);
}

#[test]
#[should_panic]
fn test_nonzero_bitmask_word_should_not_allow_zero() {
    let _zero_value = NonZeroBitMaskWord::new_unchecked(0);
} 

#[test]
fn test_nonzero_trailing_zeros_arm() {
    // This test is illustrative; the actual behavior depends on target architecture.
    // To effectively test the ARM case, this should run in an ARM environment. 
    #[cfg(target_arch = "arm")]
    {
        let nonzero_value = NonZeroBitMaskWord::new_unchecked(0b00000001);
        assert_eq!(nonzero_trailing_zeros(nonzero_value), 0);
    }
}

