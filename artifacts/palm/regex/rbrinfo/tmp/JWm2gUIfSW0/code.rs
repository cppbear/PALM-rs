fn write_varu32(data: &mut Vec<u8>, mut n: u32) {
    while n >= 0b1000_0000 {
        data.push((n as u8) | 0b1000_0000);
        n >>= 7;
    }
    data.push(n as u8);
}