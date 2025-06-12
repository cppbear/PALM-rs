pub const fn encoded_len(bytes_len: usize, padding: bool) -> Option<usize> {
    let rem = bytes_len % 3;

    let complete_input_chunks = bytes_len / 3;
    // `?` is disallowed in const, and `let Some(_) = _ else` requires 1.65.0, whereas this
    // messier syntax works on 1.48
    let complete_chunk_output =
        if let Some(complete_chunk_output) = complete_input_chunks.checked_mul(4) {
            complete_chunk_output
        } else {
            return None;
        };

    if rem > 0 {
        if padding {
            complete_chunk_output.checked_add(4)
        } else {
            let encoded_rem = match rem {
                1 => 2,
                // only other possible remainder is 2
                // can't use a separate _ => unreachable!() in const fns in ancient rust versions
                _ => 3,
            };
            complete_chunk_output.checked_add(encoded_rem)
        }
    } else {
        Some(complete_chunk_output)
    }
}