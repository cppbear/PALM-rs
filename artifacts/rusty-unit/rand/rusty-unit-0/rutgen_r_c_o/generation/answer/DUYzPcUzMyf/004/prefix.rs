// Answer 0

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

pub struct TestWeight(u8);

impl Clone for TestWeight {
    fn clone(&self) -> Self {
        TestWeight(self.0)
    }
}

impl Weight for TestWeight {
    const ZERO: Self = TestWeight(0);
    
    fn checked_add_assign(&mut self, v: &Self) -> Result<(), ()> {
        let result = self.0.checked_add(v.0);
        if result.is_some() {
            self.0 = result.unwrap();
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct TestRng;

pub trait Rng {}

impl Rng for TestRng {}

#[test]
fn test_weighted_index_overflow() {
    let weights: Vec<TestWeight> = vec![TestWeight(1), TestWeight(2), TestWeight(255)];
    let total_weight: TestWeight = weights.iter().cloned().fold(TestWeight(0), |acc, w| {
        let mut temp = acc;
        temp.checked_add_assign(&w).unwrap();
        temp
    });
    
    let result = WeightedIndex::new(weights);
    // result should be Err(Error::Overflow)
}

#[test]
fn test_weighted_index_invalid_weight_negative() {
    let weights: Vec<TestWeight> = vec![TestWeight(1), TestWeight(255), TestWeight(0), TestWeight(255), TestWeight(255), TestWeight(255)];
    let result = WeightedIndex::new(weights);
    // result should be Err(Error::InvalidWeight)
}

#[test]
fn test_weighted_index_insufficient_non_zero() {
    let weights: Vec<TestWeight> = vec![TestWeight(0), TestWeight(0)];
    let result = WeightedIndex::new(weights);
    // result should be Err(Error::InsufficientNonZero)
} 

#[test]
fn test_weighted_index_empty_input() {
    let weights: Vec<TestWeight> = vec![];
    let result = WeightedIndex::new(weights);
    // result should be Err(Error::InvalidInput)
}

