// Answer 0

#[test]
fn test_fill_u8() {
    let mut arr: [u8; 50] = [0; 50];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_i16() {
    let mut arr: [i16; 100] = [0; 100];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_u32() {
    let mut arr: [u32; 10] = [0; 10];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_i64() {
    let mut arr: [i64; 5] = [0; 5];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_bool_array() {
    let mut arr: [bool; 10] = [false; 10];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_char_array() {
    let mut arr: [char; 100] = [' '; 100];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_f32_array() {
    let mut arr: [f32; 10] = [0.0; 10];
    arr.fill(&mut rand::thread_rng());
}

#[test]
fn test_fill_f64_array() {
    let mut arr: [f64; 10] = [0.0; 10];
    arr.fill(&mut rand::thread_rng());
}

#[test]
#[should_panic]
fn test_fill_empty_bool_array() {
    let mut arr: [bool; 0] = [];
    arr.fill(&mut rand::thread_rng());
}

#[test]
#[should_panic]
fn test_fill_empty_char_array() {
    let mut arr: [char; 0] = [];
    arr.fill(&mut rand::thread_rng());
}

