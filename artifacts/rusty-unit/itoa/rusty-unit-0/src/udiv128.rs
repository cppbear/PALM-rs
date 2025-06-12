#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/// Multiply unsigned 128 bit integers, return upper 128 bits of the result
#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
fn u128_mulhi(x: u128, y: u128) -> u128 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let y_lo = y as u64;
    let y_hi = (y >> 64) as u64;

    // handle possibility of overflow
    let carry = (x_lo as u128 * y_lo as u128) >> 64;
    let m = x_lo as u128 * y_hi as u128 + carry;
    let high1 = m >> 64;

    let m_lo = m as u64;
    let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

    x_hi as u128 * y_hi as u128 + high1 + high2
}

/// Divide `n` by 1e19 and return quotient and remainder
///
/// Integer division algorithm is based on the following paper:
///
///   T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”
///   in Proc. of the SIGPLAN94 Conference on Programming Language Design and
///   Implementation, 1994, pp. 61–72
///
#[inline]
#[cfg_attr(feature = "no-panic", no_panic)]
pub fn udivmod_1e19(n: u128) -> (u128, u64) {
    let d = 10_000_000_000_000_000_000_u64; // 10^19

    let quot = if n < 1 << 83 {
        ((n >> 19) as u64 / (d >> 19)) as u128
    } else {
        u128_mulhi(n, 156927543384667019095894735580191660403) >> 62
    };

    let rem = (n - quot * d as u128) as u64;
    debug_assert_eq!(quot, n / d as u128);
    debug_assert_eq!(rem as u128, n % d as u128);

    (quot, rem)
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut u128_0: u128 = 7880u128;
    let mut u128_1: u128 = 946u128;
    let mut u128_2: u128 = 7801u128;
    let mut u128_3: u128 = 6271u128;
    let mut u128_4: u128 = 9817u128;
    let mut u128_5: u128 = 1720u128;
    let mut u128_6: u128 = 4270u128;
    let mut u128_7: u128 = 2479u128;
    let mut u128_8: u128 = 4375u128;
    let mut u128_9: u128 = 4428u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut u128_0: u128 = 9745u128;
    let mut u128_1: u128 = 4263u128;
    let mut u128_2: u128 = 859u128;
    let mut u128_3: u128 = 7661u128;
    let mut u128_4: u128 = 4377u128;
    let mut u128_5: u128 = 2848u128;
    let mut u128_6: u128 = 2133u128;
    let mut u128_7: u128 = 9835u128;
    let mut u128_8: u128 = 5200u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_9: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::default();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut u128_0: u128 = 267u128;
    let mut u128_1: u128 = 9642u128;
    let mut u128_2: u128 = 1084u128;
    let mut u128_3: u128 = 8034u128;
    let mut u128_4: u128 = 3844u128;
    let mut u128_5: u128 = 5274u128;
    let mut u128_6: u128 = 4170u128;
    let mut u128_7: u128 = 6298u128;
    let mut u128_8: u128 = 8351u128;
    let mut u128_9: u128 = 1379u128;
    let mut u128_10: u128 = 2634u128;
    let mut u128_11: u128 = 7611u128;
    let mut u128_12: u128 = 7349u128;
    let mut u128_13: u128 = 595u128;
    let mut u128_14: u128 = 5186u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut u128_0: u128 = 1514u128;
    let mut u128_1: u128 = 3562u128;
    let mut u128_2: u128 = 7822u128;
    let mut u128_3: u128 = 162u128;
    let mut u128_4: u128 = 4734u128;
    let mut u128_5: u128 = 7096u128;
    let mut u128_6: u128 = 9703u128;
    let mut u128_7: u128 = 5247u128;
    let mut u128_8: u128 = 9350u128;
    let mut u128_9: u128 = 6407u128;
    let mut u128_10: u128 = 731u128;
    let mut u128_11: u128 = 1077u128;
    let mut u128_12: u128 = 5529u128;
    let mut u128_13: u128 = 7675u128;
    let mut u128_14: u128 = 5908u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u128_0: u128 = 9202u128;
    let mut u128_1: u128 = 1557u128;
    let mut u128_2: u128 = 6686u128;
    let mut u128_3: u128 = 2612u128;
    let mut u128_4: u128 = 9322u128;
    let mut u128_5: u128 = 3768u128;
    let mut u128_6: u128 = 4268u128;
    let mut u128_7: u128 = 7967u128;
    let mut u128_8: u128 = 2665u128;
    let mut u128_9: u128 = 5672u128;
    let mut u128_10: u128 = 7010u128;
    let mut u128_11: u128 = 4862u128;
    let mut u128_12: u128 = 9312u128;
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut u128_0: u128 = 3412u128;
    let mut u128_1: u128 = 9547u128;
    let mut u128_2: u128 = 7666u128;
    let mut u128_3: u128 = 801u128;
    let mut u128_4: u128 = 4930u128;
    let mut u128_5: u128 = 1174u128;
    let mut u128_6: u128 = 6661u128;
    let mut u128_7: u128 = 8853u128;
    let mut u128_8: u128 = 650u128;
    let mut u128_9: u128 = 8589u128;
    let mut u128_10: u128 = 4047u128;
    let mut u128_11: u128 = 6970u128;
    let mut u128_12: u128 = 7256u128;
    let mut u128_13: u128 = 9397u128;
    let mut u128_14: u128 = 7305u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u128_0: u128 = 2044u128;
    let mut u128_1: u128 = 7859u128;
    let mut u128_2: u128 = 7693u128;
    let mut u128_3: u128 = 8743u128;
    let mut u128_4: u128 = 2561u128;
    let mut u128_5: u128 = 8337u128;
    let mut u128_6: u128 = 6353u128;
    let mut u128_7: u128 = 384u128;
    let mut u128_8: u128 = 6850u128;
    let mut u128_9: u128 = 1007u128;
    let mut u128_10: u128 = 349u128;
    let mut u128_11: u128 = 9767u128;
    let mut u128_12: u128 = 8453u128;
    let mut u128_13: u128 = 9330u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u128_0: u128 = 43u128;
    let mut u128_1: u128 = 271u128;
    let mut u128_2: u128 = 7281u128;
    let mut u128_3: u128 = 9057u128;
    let mut u128_4: u128 = 7274u128;
    let mut u128_5: u128 = 7371u128;
    let mut u128_6: u128 = 5226u128;
    let mut u128_7: u128 = 4843u128;
    let mut u128_8: u128 = 3009u128;
    let mut u128_9: u128 = 9633u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u128_0: u128 = 4660u128;
    let mut u128_1: u128 = 7139u128;
    let mut u128_2: u128 = 5816u128;
    let mut u128_3: u128 = 3992u128;
    let mut u128_4: u128 = 5362u128;
    let mut u128_5: u128 = 7772u128;
    let mut u128_6: u128 = 3394u128;
    let mut u128_7: u128 = 3072u128;
    let mut u128_8: u128 = 5326u128;
    let mut u128_9: u128 = 2617u128;
    let mut u128_10: u128 = 5075u128;
    let mut u128_11: u128 = 1071u128;
    let mut u128_12: u128 = 8892u128;
    let mut u128_13: u128 = 4283u128;
    let mut u128_14: u128 = 946u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u128_0: u128 = 6903u128;
    let mut u128_1: u128 = 6551u128;
    let mut u128_2: u128 = 7598u128;
    let mut u128_3: u128 = 5215u128;
    let mut u128_4: u128 = 5834u128;
    let mut u128_5: u128 = 5873u128;
    let mut u128_6: u128 = 2233u128;
    let mut u128_7: u128 = 1686u128;
    let mut u128_8: u128 = 8958u128;
    let mut u128_9: u128 = 6515u128;
    let mut u128_10: u128 = 2193u128;
    let mut u128_11: u128 = 9755u128;
    let mut u128_12: u128 = 6651u128;
    let mut u128_13: u128 = 5500u128;
    let mut u128_14: u128 = 4647u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut u128_0: u128 = 8612u128;
    let mut u128_1: u128 = 3866u128;
    let mut u128_2: u128 = 8973u128;
    let mut u128_3: u128 = 3601u128;
    let mut u128_4: u128 = 9707u128;
    let mut u128_5: u128 = 9370u128;
    let mut u128_6: u128 = 7088u128;
    let mut u128_7: u128 = 1089u128;
    let mut u128_8: u128 = 8584u128;
    let mut u128_9: u128 = 6491u128;
    let mut u128_10: u128 = 8660u128;
    let mut u128_11: u128 = 8186u128;
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut u128_0: u128 = 7247u128;
    let mut u128_1: u128 = 3523u128;
    let mut u128_2: u128 = 8006u128;
    let mut u128_3: u128 = 9412u128;
    let mut u128_4: u128 = 8626u128;
    let mut u128_5: u128 = 5982u128;
    let mut u128_6: u128 = 6051u128;
    let mut u128_7: u128 = 5209u128;
    let mut u128_8: u128 = 712u128;
    let mut u128_9: u128 = 8624u128;
    let mut u128_10: u128 = 3169u128;
    let mut u128_11: u128 = 5830u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut u128_0: u128 = 9985u128;
    let mut u128_1: u128 = 6099u128;
    let mut u128_2: u128 = 5395u128;
    let mut u128_3: u128 = 7648u128;
    let mut u128_4: u128 = 154u128;
    let mut u128_5: u128 = 1283u128;
    let mut u128_6: u128 = 8348u128;
    let mut u128_7: u128 = 3969u128;
    let mut u128_8: u128 = 175u128;
    let mut u128_9: u128 = 1683u128;
    let mut u128_10: u128 = 4740u128;
    let mut u128_11: u128 = 9677u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut u128_0: u128 = 7596u128;
    let mut u128_1: u128 = 1332u128;
    let mut u128_2: u128 = 8781u128;
    let mut u128_3: u128 = 9917u128;
    let mut u128_4: u128 = 3339u128;
    let mut u128_5: u128 = 5576u128;
    let mut u128_6: u128 = 148u128;
    let mut u128_7: u128 = 2131u128;
    let mut u128_8: u128 = 9370u128;
    let mut u128_9: u128 = 6593u128;
    let mut u128_10: u128 = 5899u128;
    let mut u128_11: u128 = 5585u128;
    let mut u128_12: u128 = 3326u128;
    let mut u128_13: u128 = 4467u128;
    let mut u128_14: u128 = 9614u128;
    let mut u128_15: u128 = 3607u128;
    let mut u128_16: u128 = 5729u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_16);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_15, u128_14);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_22: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut u128_0: u128 = 8625u128;
    let mut u128_1: u128 = 685u128;
    let mut u128_2: u128 = 2781u128;
    let mut u128_3: u128 = 3273u128;
    let mut u128_4: u128 = 6027u128;
    let mut u128_5: u128 = 9844u128;
    let mut u128_6: u128 = 5018u128;
    let mut u128_7: u128 = 1770u128;
    let mut u128_8: u128 = 5549u128;
    let mut u128_9: u128 = 8274u128;
    let mut u128_10: u128 = 9194u128;
    let mut u128_11: u128 = 2879u128;
    let mut u128_12: u128 = 6528u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u128_0: u128 = 4633u128;
    let mut u128_1: u128 = 1407u128;
    let mut u128_2: u128 = 6858u128;
    let mut u128_3: u128 = 860u128;
    let mut u128_4: u128 = 1423u128;
    let mut u128_5: u128 = 4467u128;
    let mut u128_6: u128 = 7083u128;
    let mut u128_7: u128 = 2948u128;
    let mut u128_8: u128 = 3113u128;
    let mut u128_9: u128 = 4572u128;
    let mut u128_10: u128 = 8524u128;
    let mut u128_11: u128 = 3373u128;
    let mut u128_12: u128 = 6841u128;
    let mut u128_13: u128 = 2554u128;
    let mut u128_14: u128 = 5906u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut u128_0: u128 = 4323u128;
    let mut u128_1: u128 = 4966u128;
    let mut u128_2: u128 = 3890u128;
    let mut u128_3: u128 = 9558u128;
    let mut u128_4: u128 = 6859u128;
    let mut u128_5: u128 = 574u128;
    let mut u128_6: u128 = 47u128;
    let mut u128_7: u128 = 7948u128;
    let mut u128_8: u128 = 3591u128;
    let mut u128_9: u128 = 8709u128;
    let mut u128_10: u128 = 6960u128;
    let mut u128_11: u128 = 4973u128;
    let mut u128_12: u128 = 6564u128;
    let mut u128_13: u128 = 5180u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut u128_0: u128 = 5060u128;
    let mut u128_1: u128 = 6660u128;
    let mut u128_2: u128 = 5391u128;
    let mut u128_3: u128 = 9764u128;
    let mut u128_4: u128 = 5702u128;
    let mut u128_5: u128 = 2523u128;
    let mut u128_6: u128 = 4175u128;
    let mut u128_7: u128 = 4428u128;
    let mut u128_8: u128 = 2720u128;
    let mut u128_9: u128 = 1699u128;
    let mut u128_10: u128 = 888u128;
    let mut u128_11: u128 = 2531u128;
    let mut u128_12: u128 = 932u128;
    let mut u128_13: u128 = 5519u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut u128_0: u128 = 6229u128;
    let mut u128_1: u128 = 7673u128;
    let mut u128_2: u128 = 2455u128;
    let mut u128_3: u128 = 4420u128;
    let mut u128_4: u128 = 6958u128;
    let mut u128_5: u128 = 8275u128;
    let mut u128_6: u128 = 7410u128;
    let mut u128_7: u128 = 1701u128;
    let mut u128_8: u128 = 2401u128;
    let mut u128_9: u128 = 6112u128;
    let mut u128_10: u128 = 5666u128;
    let mut u128_11: u128 = 4233u128;
    let mut u128_12: u128 = 6294u128;
    let mut u128_13: u128 = 2164u128;
    let mut u128_14: u128 = 7063u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut u128_0: u128 = 6303u128;
    let mut u128_1: u128 = 1622u128;
    let mut u128_2: u128 = 8038u128;
    let mut u128_3: u128 = 8695u128;
    let mut u128_4: u128 = 4095u128;
    let mut u128_5: u128 = 225u128;
    let mut u128_6: u128 = 4573u128;
    let mut u128_7: u128 = 5916u128;
    let mut u128_8: u128 = 1835u128;
    let mut u128_9: u128 = 3677u128;
    let mut u128_10: u128 = 6373u128;
    let mut u128_11: u128 = 1490u128;
    let mut u128_12: u128 = 6225u128;
    let mut u128_13: u128 = 583u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u128_0: u128 = 6667u128;
    let mut u128_1: u128 = 9302u128;
    let mut u128_2: u128 = 2316u128;
    let mut u128_3: u128 = 9443u128;
    let mut u128_4: u128 = 5373u128;
    let mut u128_5: u128 = 8930u128;
    let mut u128_6: u128 = 8191u128;
    let mut u128_7: u128 = 8888u128;
    let mut u128_8: u128 = 5362u128;
    let mut u128_9: u128 = 5713u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut buffer_13: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut u128_0: u128 = 6405u128;
    let mut u128_1: u128 = 2307u128;
    let mut u128_2: u128 = 8823u128;
    let mut u128_3: u128 = 8529u128;
    let mut u128_4: u128 = 8705u128;
    let mut u128_5: u128 = 94u128;
    let mut u128_6: u128 = 6941u128;
    let mut u128_7: u128 = 2832u128;
    let mut u128_8: u128 = 9891u128;
    let mut u128_9: u128 = 4976u128;
    let mut u128_10: u128 = 1377u128;
    let mut u128_11: u128 = 397u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u128_0: u128 = 4412u128;
    let mut u128_1: u128 = 816u128;
    let mut u128_2: u128 = 4191u128;
    let mut u128_3: u128 = 4837u128;
    let mut u128_4: u128 = 1208u128;
    let mut u128_5: u128 = 1296u128;
    let mut u128_6: u128 = 8375u128;
    let mut u128_7: u128 = 6048u128;
    let mut u128_8: u128 = 6999u128;
    let mut u128_9: u128 = 7319u128;
    let mut u128_10: u128 = 7764u128;
    let mut u128_11: u128 = 307u128;
    let mut u128_12: u128 = 2853u128;
    let mut u128_13: u128 = 5378u128;
    let mut u128_14: u128 = 319u128;
    let mut u128_15: u128 = 5880u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut u128_0: u128 = 9205u128;
    let mut u128_1: u128 = 6705u128;
    let mut u128_2: u128 = 1872u128;
    let mut u128_3: u128 = 2424u128;
    let mut u128_4: u128 = 9712u128;
    let mut u128_5: u128 = 8361u128;
    let mut u128_6: u128 = 8902u128;
    let mut u128_7: u128 = 6442u128;
    let mut u128_8: u128 = 7262u128;
    let mut u128_9: u128 = 3188u128;
    let mut u128_10: u128 = 3601u128;
    let mut u128_11: u128 = 5367u128;
    let mut u128_12: u128 = 8901u128;
    let mut u128_13: u128 = 3438u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u128_0: u128 = 6368u128;
    let mut u128_1: u128 = 7573u128;
    let mut u128_2: u128 = 771u128;
    let mut u128_3: u128 = 6807u128;
    let mut u128_4: u128 = 6802u128;
    let mut u128_5: u128 = 6306u128;
    let mut u128_6: u128 = 1981u128;
    let mut u128_7: u128 = 6558u128;
    let mut u128_8: u128 = 9582u128;
    let mut u128_9: u128 = 3215u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut u128_10: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_10: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_11: crate::Buffer = crate::Buffer::default();
    let mut buffer_12: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut u128_0: u128 = 9098u128;
    let mut u128_1: u128 = 3265u128;
    let mut u128_2: u128 = 9312u128;
    let mut u128_3: u128 = 7779u128;
    let mut u128_4: u128 = 4646u128;
    let mut u128_5: u128 = 6168u128;
    let mut u128_6: u128 = 952u128;
    let mut u128_7: u128 = 2506u128;
    let mut u128_8: u128 = 7160u128;
    let mut u128_9: u128 = 5214u128;
    let mut u128_10: u128 = 6425u128;
    let mut u128_11: u128 = 8353u128;
    let mut u128_12: u128 = 699u128;
    let mut u128_13: u128 = 7389u128;
    let mut u128_14: u128 = 2931u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut u128_0: u128 = 5666u128;
    let mut u128_1: u128 = 3117u128;
    let mut u128_2: u128 = 2016u128;
    let mut u128_3: u128 = 3488u128;
    let mut u128_4: u128 = 8219u128;
    let mut u128_5: u128 = 8485u128;
    let mut u128_6: u128 = 3992u128;
    let mut u128_7: u128 = 9120u128;
    let mut u128_8: u128 = 3628u128;
    let mut u128_9: u128 = 9735u128;
    let mut u128_10: u128 = 8576u128;
    let mut u128_11: u128 = 1916u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut u128_0: u128 = 6029u128;
    let mut u128_1: u128 = 5495u128;
    let mut u128_2: u128 = 2603u128;
    let mut u128_3: u128 = 1114u128;
    let mut u128_4: u128 = 1420u128;
    let mut u128_5: u128 = 1970u128;
    let mut u128_6: u128 = 9302u128;
    let mut u128_7: u128 = 1560u128;
    let mut u128_8: u128 = 8166u128;
    let mut u128_9: u128 = 2761u128;
    let mut u128_10: u128 = 9062u128;
    let mut u128_11: u128 = 3722u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut u128_0: u128 = 8820u128;
    let mut u128_1: u128 = 6854u128;
    let mut u128_2: u128 = 4613u128;
    let mut u128_3: u128 = 811u128;
    let mut u128_4: u128 = 3198u128;
    let mut u128_5: u128 = 9298u128;
    let mut u128_6: u128 = 3500u128;
    let mut u128_7: u128 = 8411u128;
    let mut u128_8: u128 = 9563u128;
    let mut u128_9: u128 = 8505u128;
    let mut u128_10: u128 = 785u128;
    let mut u128_11: u128 = 6272u128;
    let mut u128_12: u128 = 8543u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut u128_0: u128 = 2228u128;
    let mut u128_1: u128 = 2231u128;
    let mut u128_2: u128 = 5859u128;
    let mut u128_3: u128 = 3620u128;
    let mut u128_4: u128 = 6351u128;
    let mut u128_5: u128 = 4990u128;
    let mut u128_6: u128 = 2826u128;
    let mut u128_7: u128 = 7859u128;
    let mut u128_8: u128 = 8226u128;
    let mut u128_9: u128 = 3557u128;
    let mut u128_10: u128 = 3554u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    let mut buffer_12: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut u128_0: u128 = 579u128;
    let mut u128_1: u128 = 3936u128;
    let mut u128_2: u128 = 4596u128;
    let mut u128_3: u128 = 4221u128;
    let mut u128_4: u128 = 4979u128;
    let mut u128_5: u128 = 9720u128;
    let mut u128_6: u128 = 8214u128;
    let mut u128_7: u128 = 2703u128;
    let mut u128_8: u128 = 8891u128;
    let mut u128_9: u128 = 8403u128;
    let mut u128_10: u128 = 789u128;
    let mut u128_11: u128 = 2033u128;
    let mut u128_12: u128 = 8193u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut u128_0: u128 = 7951u128;
    let mut u128_1: u128 = 4193u128;
    let mut u128_2: u128 = 2254u128;
    let mut u128_3: u128 = 1513u128;
    let mut u128_4: u128 = 5070u128;
    let mut u128_5: u128 = 5986u128;
    let mut u128_6: u128 = 9156u128;
    let mut u128_7: u128 = 2285u128;
    let mut u128_8: u128 = 3822u128;
    let mut u128_9: u128 = 1199u128;
    let mut u128_10: u128 = 4566u128;
    let mut u128_11: u128 = 216u128;
    let mut u128_12: u128 = 5825u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut u128_0: u128 = 8438u128;
    let mut u128_1: u128 = 4790u128;
    let mut u128_2: u128 = 6227u128;
    let mut u128_3: u128 = 6195u128;
    let mut u128_4: u128 = 1060u128;
    let mut u128_5: u128 = 2248u128;
    let mut u128_6: u128 = 2508u128;
    let mut u128_7: u128 = 3833u128;
    let mut u128_8: u128 = 8153u128;
    let mut u128_9: u128 = 1824u128;
    let mut u128_10: u128 = 8390u128;
    let mut u128_11: u128 = 3603u128;
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut u128_0: u128 = 2776u128;
    let mut u128_1: u128 = 711u128;
    let mut u128_2: u128 = 5293u128;
    let mut u128_3: u128 = 1047u128;
    let mut u128_4: u128 = 1257u128;
    let mut u128_5: u128 = 7598u128;
    let mut u128_6: u128 = 2949u128;
    let mut u128_7: u128 = 5597u128;
    let mut u128_8: u128 = 8727u128;
    let mut u128_9: u128 = 8535u128;
    let mut u128_10: u128 = 3917u128;
    let mut u128_11: u128 = 4664u128;
    let mut u128_12: u128 = 7135u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u128_0: u128 = 1586u128;
    let mut u128_1: u128 = 3151u128;
    let mut u128_2: u128 = 6597u128;
    let mut u128_3: u128 = 5382u128;
    let mut u128_4: u128 = 3134u128;
    let mut u128_5: u128 = 8220u128;
    let mut u128_6: u128 = 808u128;
    let mut u128_7: u128 = 9737u128;
    let mut u128_8: u128 = 3475u128;
    let mut u128_9: u128 = 182u128;
    let mut u128_10: u128 = 7952u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut u128_0: u128 = 346u128;
    let mut u128_1: u128 = 2162u128;
    let mut u128_2: u128 = 6151u128;
    let mut u128_3: u128 = 1186u128;
    let mut u128_4: u128 = 5241u128;
    let mut u128_5: u128 = 8326u128;
    let mut u128_6: u128 = 2542u128;
    let mut u128_7: u128 = 9191u128;
    let mut u128_8: u128 = 9292u128;
    let mut u128_9: u128 = 7850u128;
    let mut u128_10: u128 = 6782u128;
    let mut u128_11: u128 = 3793u128;
    let mut u128_12: u128 = 4389u128;
    let mut u128_13: u128 = 406u128;
    let mut u128_14: u128 = 873u128;
    let mut u128_15: u128 = 4064u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u128_0: u128 = 5827u128;
    let mut u128_1: u128 = 7601u128;
    let mut u128_2: u128 = 7884u128;
    let mut u128_3: u128 = 7565u128;
    let mut u128_4: u128 = 5596u128;
    let mut u128_5: u128 = 765u128;
    let mut u128_6: u128 = 5311u128;
    let mut u128_7: u128 = 4428u128;
    let mut u128_8: u128 = 7981u128;
    let mut u128_9: u128 = 2114u128;
    let mut u128_10: u128 = 4516u128;
    let mut u128_11: u128 = 4849u128;
    let mut u128_12: u128 = 7960u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_6: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_7: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_8: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut u128_0: u128 = 2797u128;
    let mut u128_1: u128 = 5804u128;
    let mut u128_2: u128 = 163u128;
    let mut u128_3: u128 = 5718u128;
    let mut u128_4: u128 = 9441u128;
    let mut u128_5: u128 = 8137u128;
    let mut u128_6: u128 = 9639u128;
    let mut u128_7: u128 = 3186u128;
    let mut u128_8: u128 = 5398u128;
    let mut u128_9: u128 = 7857u128;
    let mut u128_10: u128 = 4022u128;
    let mut u128_11: u128 = 6193u128;
    let mut u128_12: u128 = 8123u128;
    let mut u128_13: u128 = 6907u128;
    let mut u128_14: u128 = 9078u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_7, u128_6);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut u128_0: u128 = 6342u128;
    let mut u128_1: u128 = 99u128;
    let mut u128_2: u128 = 5195u128;
    let mut u128_3: u128 = 5344u128;
    let mut u128_4: u128 = 8741u128;
    let mut u128_5: u128 = 7383u128;
    let mut u128_6: u128 = 2660u128;
    let mut u128_7: u128 = 3394u128;
    let mut u128_8: u128 = 5017u128;
    let mut u128_9: u128 = 7790u128;
    let mut u128_10: u128 = 7341u128;
    let mut u128_11: u128 = 8645u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut u128_0: u128 = 4744u128;
    let mut u128_1: u128 = 1433u128;
    let mut u128_2: u128 = 3056u128;
    let mut u128_3: u128 = 6701u128;
    let mut u128_4: u128 = 1704u128;
    let mut u128_5: u128 = 7160u128;
    let mut u128_6: u128 = 9412u128;
    let mut u128_7: u128 = 3015u128;
    let mut u128_8: u128 = 694u128;
    let mut u128_9: u128 = 9012u128;
    let mut u128_10: u128 = 6580u128;
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut u128_0: u128 = 4190u128;
    let mut u128_1: u128 = 5357u128;
    let mut u128_2: u128 = 9895u128;
    let mut u128_3: u128 = 9325u128;
    let mut u128_4: u128 = 4069u128;
    let mut u128_5: u128 = 1126u128;
    let mut u128_6: u128 = 7275u128;
    let mut u128_7: u128 = 898u128;
    let mut u128_8: u128 = 1975u128;
    let mut u128_9: u128 = 4654u128;
    let mut u128_10: u128 = 8904u128;
    let mut u128_11: u128 = 9850u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut u128_0: u128 = 3663u128;
    let mut u128_1: u128 = 1246u128;
    let mut u128_2: u128 = 7472u128;
    let mut u128_3: u128 = 6201u128;
    let mut u128_4: u128 = 6044u128;
    let mut u128_5: u128 = 6312u128;
    let mut u128_6: u128 = 6889u128;
    let mut u128_7: u128 = 1250u128;
    let mut u128_8: u128 = 1814u128;
    let mut u128_9: u128 = 4543u128;
    let mut u128_10: u128 = 3513u128;
    let mut u128_11: u128 = 691u128;
    let mut u128_12: u128 = 8426u128;
    let mut u128_13: u128 = 6179u128;
    let mut u128_14: u128 = 2162u128;
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u128_0: u128 = 5793u128;
    let mut u128_1: u128 = 9720u128;
    let mut u128_2: u128 = 2770u128;
    let mut u128_3: u128 = 10u128;
    let mut u128_4: u128 = 1465u128;
    let mut u128_5: u128 = 9118u128;
    let mut u128_6: u128 = 3181u128;
    let mut u128_7: u128 = 7053u128;
    let mut u128_8: u128 = 9320u128;
    let mut u128_9: u128 = 9261u128;
    let mut u128_10: u128 = 3331u128;
    let mut u128_11: u128 = 5512u128;
    let mut u128_12: u128 = 397u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_12, u128_11);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut buffer_8: crate::Buffer = crate::Buffer::default();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut u128_0: u128 = 110u128;
    let mut u128_1: u128 = 9677u128;
    let mut u128_2: u128 = 9973u128;
    let mut u128_3: u128 = 8985u128;
    let mut u128_4: u128 = 3294u128;
    let mut u128_5: u128 = 338u128;
    let mut u128_6: u128 = 7340u128;
    let mut u128_7: u128 = 5560u128;
    let mut u128_8: u128 = 1753u128;
    let mut u128_9: u128 = 883u128;
    let mut u128_10: u128 = 4804u128;
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_5, u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_3);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_2, u128_1);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::new();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut u128_0: u128 = 9131u128;
    let mut u128_1: u128 = 5766u128;
    let mut u128_2: u128 = 4179u128;
    let mut u128_3: u128 = 3600u128;
    let mut u128_4: u128 = 4259u128;
    let mut u128_5: u128 = 739u128;
    let mut u128_6: u128 = 6855u128;
    let mut u128_7: u128 = 6281u128;
    let mut u128_8: u128 = 8035u128;
    let mut u128_9: u128 = 8561u128;
    let mut u128_10: u128 = 2823u128;
    let mut u128_11: u128 = 6741u128;
    let mut u128_12: u128 = 4761u128;
    let mut u128_13: u128 = 2355u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut buffer_2: crate::Buffer = crate::Buffer::default();
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_1);
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut u128_0: u128 = 9237u128;
    let mut u128_1: u128 = 9771u128;
    let mut u128_2: u128 = 5854u128;
    let mut u128_3: u128 = 9388u128;
    let mut u128_4: u128 = 3773u128;
    let mut u128_5: u128 = 5373u128;
    let mut u128_6: u128 = 8843u128;
    let mut u128_7: u128 = 8568u128;
    let mut u128_8: u128 = 1436u128;
    let mut u128_9: u128 = 8269u128;
    let mut u128_10: u128 = 8809u128;
    let mut u128_11: u128 = crate::udiv128::u128_mulhi(u128_10, u128_9);
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_8, u128_7);
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_6);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_5);
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut buffer_9: crate::Buffer = crate::Buffer::default();
    let mut buffer_10: crate::Buffer = crate::Buffer::default();
    let mut buffer_11: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut u128_0: u128 = 3024u128;
    let mut u128_1: u128 = 8300u128;
    let mut u128_2: u128 = 8428u128;
    let mut u128_3: u128 = 8590u128;
    let mut u128_4: u128 = 280u128;
    let mut u128_5: u128 = 3180u128;
    let mut u128_6: u128 = 5293u128;
    let mut u128_7: u128 = 9284u128;
    let mut u128_8: u128 = 7905u128;
    let mut u128_9: u128 = 5804u128;
    let mut u128_10: u128 = 6916u128;
    let mut u128_11: u128 = 5751u128;
    let mut u128_12: u128 = 1935u128;
    let mut u128_13: u128 = 7507u128;
    let mut u128_14: u128 = 4673u128;
    let mut u128_15: u128 = 1881u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_15);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_14, u128_13);
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_12);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut u128_21: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_3: crate::Buffer = crate::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u128_0: u128 = 518u128;
    let mut u128_1: u128 = 9602u128;
    let mut u128_2: u128 = 5923u128;
    let mut u128_3: u128 = 5845u128;
    let mut u128_4: u128 = 9264u128;
    let mut u128_5: u128 = 9842u128;
    let mut u128_6: u128 = 2532u128;
    let mut u128_7: u128 = 8038u128;
    let mut u128_8: u128 = 6773u128;
    let mut u128_9: u128 = 6842u128;
    let mut u128_10: u128 = 5322u128;
    let mut u128_11: u128 = 3002u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_11);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_10);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_9);
    let mut tuple_3: (u128, u64) = crate::udiv128::udivmod_1e19(u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::new();
    let mut buffer_7: crate::Buffer = crate::Buffer::new();
    let mut tuple_4: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_12: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut u128_13: u128 = crate::udiv128::u128_mulhi(u128_4, u128_3);
    let mut tuple_5: (u128, u64) = crate::udiv128::udivmod_1e19(u128_2);
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u128_0: u128 = 6849u128;
    let mut u128_1: u128 = 5538u128;
    let mut u128_2: u128 = 4913u128;
    let mut u128_3: u128 = 2598u128;
    let mut u128_4: u128 = 1680u128;
    let mut u128_5: u128 = 8360u128;
    let mut u128_6: u128 = 1917u128;
    let mut u128_7: u128 = 3990u128;
    let mut u128_8: u128 = 8352u128;
    let mut u128_9: u128 = 1482u128;
    let mut u128_10: u128 = 6947u128;
    let mut u128_11: u128 = 4500u128;
    let mut u128_12: u128 = 7146u128;
    let mut u128_13: u128 = 3460u128;
    let mut u128_14: u128 = 1227u128;
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_14);
    let mut buffer_0: crate::Buffer = crate::Buffer::new();
    let mut buffer_1: crate::Buffer = crate::Buffer::new();
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut tuple_2: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut u128_20: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut buffer_4: crate::Buffer = crate::Buffer::default();
    let mut buffer_5: crate::Buffer = crate::Buffer::new();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut u128_0: u128 = 8706u128;
    let mut u128_1: u128 = 1818u128;
    let mut u128_2: u128 = 6803u128;
    let mut u128_3: u128 = 1434u128;
    let mut u128_4: u128 = 8115u128;
    let mut u128_5: u128 = 3794u128;
    let mut u128_6: u128 = 7543u128;
    let mut u128_7: u128 = 6252u128;
    let mut u128_8: u128 = 1687u128;
    let mut u128_9: u128 = 2737u128;
    let mut u128_10: u128 = 8974u128;
    let mut u128_11: u128 = 2512u128;
    let mut u128_12: u128 = 7888u128;
    let mut u128_13: u128 = 9160u128;
    let mut u128_14: u128 = crate::udiv128::u128_mulhi(u128_13, u128_12);
    let mut u128_15: u128 = crate::udiv128::u128_mulhi(u128_11, u128_10);
    let mut u128_16: u128 = crate::udiv128::u128_mulhi(u128_9, u128_8);
    let mut buffer_0: crate::Buffer = crate::Buffer::default();
    let mut buffer_1: crate::Buffer = crate::Buffer::default();
    let mut buffer_2: crate::Buffer = crate::Buffer::new();
    let mut buffer_3: crate::Buffer = crate::Buffer::new();
    let mut tuple_0: (u128, u64) = crate::udiv128::udivmod_1e19(u128_7);
    let mut buffer_4: crate::Buffer = crate::Buffer::new();
    let mut u128_17: u128 = crate::udiv128::u128_mulhi(u128_6, u128_5);
    let mut buffer_5: crate::Buffer = crate::Buffer::default();
    let mut buffer_6: crate::Buffer = crate::Buffer::default();
    let mut tuple_1: (u128, u64) = crate::udiv128::udivmod_1e19(u128_4);
    let mut buffer_7: crate::Buffer = crate::Buffer::default();
    let mut u128_18: u128 = crate::udiv128::u128_mulhi(u128_3, u128_2);
    let mut buffer_8: crate::Buffer = crate::Buffer::new();
    let mut u128_19: u128 = crate::udiv128::u128_mulhi(u128_1, u128_0);
    panic!("From RustyUnit with love");
}
}