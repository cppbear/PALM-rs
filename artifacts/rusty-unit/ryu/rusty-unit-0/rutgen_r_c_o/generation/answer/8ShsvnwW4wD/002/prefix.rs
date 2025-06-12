// Answer 0

#[test]
fn test_mul_pow5_div_pow2_panic_on_i_boundary() {
    let m: u32 = 0; 
    let i: u32 = d2s::DOUBLE_POW5_SPLIT.len() as u32; 
    let j: i32 = 33; 
    unsafe {
        let _ = mul_pow5_div_pow2(m, i, j);
    }
}

#[test]
fn test_mul_pow5_div_pow2_panic_on_i_boundary_max() {
    let m: u32 = u32::MAX; 
    let i: u32 = d2s::DOUBLE_POW5_SPLIT.len() as u32; 
    let j: i32 = i32::MAX; 
    unsafe {
        let _ = mul_pow5_div_pow2(m, i, j);
    }
}

