// Answer 0

#[derive(Debug)]
struct Uniform<T> {
    low: T,
    high: T,
}

impl<T> Uniform<T> {
    fn new<B1, B2>(low: B1, high: B2) -> Result<Uniform<T>, String>
    where
        B1: SampleBorrow<T> + Sized,
        B2: SampleBorrow<T> + Sized,
        T: PartialOrd + Copy + std::fmt::Debug,
    {
        let low_val = low.borrow();
        let high_val = high.borrow();
        
        if low_val >= high_val {
            return Err("low must be less than high".to_string());
        }
        
        Ok(Uniform { low: *low_val, high: *high_val })
    }
}

#[derive(Debug)]
struct SampleBorrowWrapper<T> {
    value: T,
}

impl<T> SampleBorrow<T> for SampleBorrowWrapper<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

#[test]
fn test_uniform_new_valid() {
    let low = SampleBorrowWrapper { value: 1 };
    let high = SampleBorrowWrapper { value: 5 };
    
    let uniform = Uniform::<i32>::new(low, high).unwrap();
    
    assert_eq!(uniform.low, 1);
    assert_eq!(uniform.high, 5);
}

#[test]
#[should_panic(expected = "low must be less than high")]
fn test_uniform_new_invalid_low_ge_high() {
    let low = SampleBorrowWrapper { value: 5 };
    let high = SampleBorrowWrapper { value: 5 };
    
    let _ = Uniform::<i32>::new(low, high).unwrap();
}

#[test]
#[should_panic(expected = "low must be less than high")]
fn test_uniform_new_invalid_low_eq_high() {
    let low = SampleBorrowWrapper { value: 3 };
    let high = SampleBorrowWrapper { value: 3 };
    
    let _ = Uniform::<i32>::new(low, high).unwrap();
}

