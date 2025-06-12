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
mod tests_llm_16_17 {
    use crate::udiv128::u128_mulhi;

    #[test]
    fn test_u128_mulhi() {
        assert_eq!(u128_mulhi(10u128, 20u128), 0); // 10 * 20 = 200, upper 128 bits = 0
        assert_eq!(u128_mulhi(0u128, 0u128), 0); // 0 * 0 = 0, upper 128 bits = 0
        assert_eq!(u128_mulhi(u128::MAX, 1u128), 0); // MAX * 1 = MAX, upper 128 bits = 0
        assert_eq!(u128_mulhi(u128::MAX, 2u128), 1); // MAX * 2 = 2*MAX, upper 128 bits = 1
        assert_eq!(u128_mulhi(1u128 << 128 - 1, 1u128 << 128 - 1), 1); // (2^127)*(2^127) = 2^254, upper 128 bits = 1
    }
}

#[cfg(test)]
mod tests_llm_16_18 {
    use crate::udiv128::udivmod_1e19;

    #[test]
    fn test_udivmod_1e19() {
        // Test cases
        let cases = [
            (0, (0, 0)),
            (10_000_000_000_000_000_000, (1, 0)),
            (19_000_000_000_000_000_000, (1, 900000000000000000)),
            (10_101_000_000_000_000_000, (1, 101000000000000000)),
            (1_000_000_000_000_000_000_000, (100, 0)),
            (1_234_567_890_123_456_789_123, (123, 456789123)),
            (u128::MAX, (u128::MAX / 10_000_000_000_000_000_000, 999999999999999999)),
        ];

        for &(n, (expected_quot, expected_rem)) in &cases {
            let (quot, rem) = udivmod_1e19(n);
            assert_eq!(quot, expected_quot);
            assert_eq!(rem, expected_rem);
        }
    }
}
