pub fn should_exec(num_insts: usize, text_len: usize) -> bool {
    // Total memory usage in bytes is determined by:
    //
    //   ((len(insts) * (len(input) + 1) + bits - 1) / bits) * (size_of(u32))
    //
    // The actual limit picked is pretty much a heuristic.
    // See: https://github.com/rust-lang/regex/issues/215
    let size = ((num_insts * (text_len + 1) + BIT_SIZE - 1) / BIT_SIZE) * 4;
    size <= MAX_SIZE_BYTES
}