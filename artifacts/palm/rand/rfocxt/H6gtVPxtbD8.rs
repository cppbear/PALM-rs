use rand_core::{impls, RngCore};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[deprecated(since = "0.9.2", note = "Deprecated without replacement")]
pub struct StepRng {
    v: u64,
    a: u64,
}
impl StepRng {
    pub fn new(initial: u64, increment: u64) -> Self {
        StepRng {
            v: initial,
            a: increment,
        }
    }
}
