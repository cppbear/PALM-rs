// Answer 0

#[test]
fn test_chunk_mut_case_1() {
    let mut vec = Vec::with_capacity(32);
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let chunk = unsafe { vec.chunk_mut() };
}

#[test]
fn test_chunk_mut_case_2() {
    let mut vec = Vec::with_capacity(10);
    vec.push(0);
    vec.push(1);
    let chunk = unsafe { vec.chunk_mut() };
}

#[test]
fn test_chunk_mut_case_3() {
    let mut vec = Vec::with_capacity(63);
    for i in 0..62 {
        vec.push(i as u8);
    }
    let chunk = unsafe { vec.chunk_mut() };
}

