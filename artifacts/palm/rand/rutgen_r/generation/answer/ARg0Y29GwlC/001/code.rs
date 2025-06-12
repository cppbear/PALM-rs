// Answer 0

#[test]
fn test_read_u64_into_success() {
    let src: Vec<u8> = (0..64).map(|x| x as u8).collect(); // 64 bytes for 8 u64
    let mut dst: [u64; 8] = [0; 8];

    read_u64_into(&src, &mut dst);

    assert_eq!(dst, [
        0x0403020106070500,
        0x0C0B0A0908070605,
        0x1413121110090807,
        0x1C1B1A1918171615,
        0x2423222120191817,
        0x2C2B2A2928272625,
        0x3433323130373635,
        0x3C3B3A3938373635,
    ]);
}

#[test]
#[should_panic]
fn test_read_u64_into_too_few_bytes() {
    let src: Vec<u8> = (0..16).map(|x| x as u8).collect(); // 16 bytes for 2 u64
    let mut dst: [u64; 4] = [0; 4];

    read_u64_into(&src, &mut dst);
}

#[test]
#[should_panic]
fn test_read_u64_into_too_many_elements() {
    let src: Vec<u8> = (0..64).map(|x| x as u8).collect(); // 64 bytes for 8 u64
    let mut dst: [u64; 9] = [0; 9]; // 9 elements, should panic

    read_u64_into(&src, &mut dst);
} 

#[test]
#[should_panic]
fn test_read_u64_into_empty_dst() {
    let src: Vec<u8> = (0..8).map(|x| x as u8).collect(); // 8 bytes for 1 u64
    let mut dst: [u64; 0] = []; // empty dst

    read_u64_into(&src, &mut dst);
}

#[test]
fn test_read_u64_into_exact_chunk() {
    let src: Vec<u8> = (0..24).map(|x| x as u8).collect(); // 24 bytes for 3 u64
    let mut dst: [u64; 3] = [0; 3];

    read_u64_into(&src, &mut dst);

    assert_eq!(dst, [
        0x0403020106070500,
        0x0C0B0A0908070605,
        0x1413121110090807,
    ]);
}

