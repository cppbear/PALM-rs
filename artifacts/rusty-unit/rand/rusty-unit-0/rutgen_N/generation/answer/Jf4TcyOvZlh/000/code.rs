// Answer 0

#[test]
fn test_output_dxsm() {
    const MULTIPLIER: u64 = 0x5851f42b4c957f2d;

    // Test case 1: Basic input
    let state1: u128 = 0x00000000000000000000000000000001;
    let result1 = output_dxsm(state1);
    let expected1: u64 = (MULTIPLIER.wrapping_mul(1)) ^ (MULTIPLIER >> 48);
    assert_eq!(result1, expected1);

    // Test case 2: State with a higher value
    let state2: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let result2 = output_dxsm(state2);
    let lo2 = state2 as u64 | 1;
    let hi2 = (state2 >> 64) as u64;
    let intermediate_hi2 = hi2 ^ (hi2 >> 32);
    let hi2 = intermediate_hi2.wrapping_mul(MULTIPLIER);
    let expected2 = hi2 ^ (hi2 >> 48).wrapping_mul(lo2);
    assert_eq!(result2, expected2);

    // Test case 3: State is zero
    let state3: u128 = 0x00000000000000000000000000000000;
    let result3 = output_dxsm(state3);
    let expected3: u64 = (MULTIPLIER.wrapping_mul(1)) ^ (MULTIPLIER >> 48);
    assert_eq!(result3, expected3);

    // Test case 4: State with maximum low part
    let state4: u128 = 0x0000000000000000FFFFFFFFFFFFFFFF;
    let result4 = output_dxsm(state4);
    let lo4 = state4 as u64 | 1;
    let hi4 = (state4 >> 64) as u64;
    let intermediate_hi4 = hi4 ^ (hi4 >> 32);
    let hi4 = intermediate_hi4.wrapping_mul(MULTIPLIER);
    let expected4 = hi4 ^ (hi4 >> 48).wrapping_mul(lo4);
    assert_eq!(result4, expected4);
}

