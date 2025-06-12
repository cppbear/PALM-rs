use crate::distr::Distribution;
use crate::Rng;
use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
const ALWAYS_TRUE: u64 = u64::MAX;
const SCALE: f64 = 2.0 * (1u64 << 63) as f64;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BernoulliError {
    /// `p < 0` or `p > 1`.
    InvalidProbability,
}
impl fmt::Display for BernoulliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                BernoulliError::InvalidProbability => {
                    "p is outside [0, 1] in Bernoulli distribution"
                }
            },
        )
    }
}
