// Answer 0

#[test]
fn test_advance_mut_no_remaining_in_a() {
    struct A {
        remaining: usize,
    }
    
    struct B {
        buffer: usize,
    }
    
    impl A {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            // Assume some logic to advance the buffer
            self.remaining = self.remaining.saturating_sub(cnt);
        }
    }
    
    impl B {
        unsafe fn advance_mut(&mut self, cnt: usize) {
            // Assume some logic to advance the buffer
            self.buffer += cnt;
        }
    }

    struct Chain {
        a: A,
        b: B,
    }

    let mut a = A { remaining: 0 }; // a_rem == 0
    let mut b = B { buffer: 0 };
    let mut chain = Chain { a, b };

    let cnt: usize = 5; // Arbitrary count to test the advanced functionality

    unsafe {
        chain.advance_mut(cnt);
    }

    // Checking the state, since a_rem is zero, we expect b's buffer to have increased by cnt
    assert_eq!(chain.b.buffer, cnt);
}

