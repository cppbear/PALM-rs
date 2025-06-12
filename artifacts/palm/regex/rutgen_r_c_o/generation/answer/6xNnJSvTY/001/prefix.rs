// Answer 0

#[test]
fn test_should_exec_zero_instructions_no_text() {
    let num_insts = 0;
    let text_len = 0;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_zero_instructions_large_text() {
    let num_insts = 0;
    let text_len = 65535;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_max_instructions_zero_text() {
    let num_insts = 2048;
    let text_len = 0;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_half_max_instructions_half_length_text() {
    let num_insts = 1024;
    let text_len = 32767;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_full_instructions_full_length_text() {
    let num_insts = 2048;
    let text_len = 65535;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_boundary_instructions_boundary_text() {
    let num_insts = 256;
    let text_len = 256;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_mid_range_instructions_mid_range_text() {
    let num_insts = 1024;
    let text_len = 1024;
    should_exec(num_insts, text_len);
}

#[test]
fn test_should_exec_max_insts_max_size_bytes() {
    let num_insts = 2048;
    let text_len = 14; // This will compute close to MAX_SIZE_BYTES
    should_exec(num_insts, text_len);
}

