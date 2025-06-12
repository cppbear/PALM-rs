fn pow5factor_32(mut value: u32) -> u32 {
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        let q = value / 5;
        let r = value % 5;
        if r != 0 {
            break;
        }
        value = q;
        count += 1;
    }
    count
}