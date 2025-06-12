fn write(&mut self, input: &[u8]) -> Result<usize> {
        assert!(
            self.delegate.is_some(),
            "Cannot write more after calling finish()"
        );

        if input.is_empty() {
            return Ok(0);
        }

        // The contract of `Write::write` places some constraints on this implementation:
        // - a call to `write()` represents at most one call to a wrapped `Write`, so we can't
        // iterate over the input and encode multiple chunks.
        // - Errors mean that "no bytes were written to this writer", so we need to reset the
        // internal state to what it was before the error occurred

        // before reading any input, write any leftover encoded output from last time
        if self.output_occupied_len > 0 {
            let current_len = self.output_occupied_len;
            return self
                .write_to_delegate(current_len)
                // did not read any input
                .map(|()| 0);
        }

        debug_assert_eq!(0, self.output_occupied_len);

        // how many bytes, if any, were read into `extra` to create a triple to encode
        let mut extra_input_read_len = 0;
        let mut input = input;

        let orig_extra_len = self.extra_input_occupied_len;

        let mut encoded_size = 0;
        // always a multiple of MIN_ENCODE_CHUNK_SIZE
        let mut max_input_len = MAX_INPUT_LEN;

        // process leftover un-encoded input from last write
        if self.extra_input_occupied_len > 0 {
            debug_assert!(self.extra_input_occupied_len < 3);
            if input.len() + self.extra_input_occupied_len >= MIN_ENCODE_CHUNK_SIZE {
                // Fill up `extra`, encode that into `output`, and consume as much of the rest of
                // `input` as possible.
                // We could write just the encoding of `extra` by itself but then we'd have to
                // return after writing only 4 bytes, which is inefficient if the underlying writer
                // would make a syscall.
                extra_input_read_len = MIN_ENCODE_CHUNK_SIZE - self.extra_input_occupied_len;
                debug_assert!(extra_input_read_len > 0);
                // overwrite only bytes that weren't already used. If we need to rollback extra_len
                // (when the subsequent write errors), the old leading bytes will still be there.
                self.extra_input[self.extra_input_occupied_len..MIN_ENCODE_CHUNK_SIZE]
                    .copy_from_slice(&input[0..extra_input_read_len]);

                let len = self.engine.internal_encode(
                    &self.extra_input[0..MIN_ENCODE_CHUNK_SIZE],
                    &mut self.output[..],
                );
                debug_assert_eq!(4, len);

                input = &input[extra_input_read_len..];

                // consider extra to be used up, since we encoded it
                self.extra_input_occupied_len = 0;
                // don't clobber where we just encoded to
                encoded_size = 4;
                // and don't read more than can be encoded
                max_input_len = MAX_INPUT_LEN - MIN_ENCODE_CHUNK_SIZE;

            // fall through to normal encoding
            } else {
                // `extra` and `input` are non empty, but `|extra| + |input| < 3`, so there must be
                // 1 byte in each.
                debug_assert_eq!(1, input.len());
                debug_assert_eq!(1, self.extra_input_occupied_len);

                self.extra_input[self.extra_input_occupied_len] = input[0];
                self.extra_input_occupied_len += 1;
                return Ok(1);
            };
        } else if input.len() < MIN_ENCODE_CHUNK_SIZE {
            // `extra` is empty, and `input` fits inside it
            self.extra_input[0..input.len()].copy_from_slice(input);
            self.extra_input_occupied_len = input.len();
            return Ok(input.len());
        };

        // either 0 or 1 complete chunks encoded from extra
        debug_assert!(encoded_size == 0 || encoded_size == 4);
        debug_assert!(
            // didn't encode extra input
            MAX_INPUT_LEN == max_input_len
                // encoded one triple
                || MAX_INPUT_LEN == max_input_len + MIN_ENCODE_CHUNK_SIZE
        );

        // encode complete triples only
        let input_complete_chunks_len = input.len() - (input.len() % MIN_ENCODE_CHUNK_SIZE);
        let input_chunks_to_encode_len = cmp::min(input_complete_chunks_len, max_input_len);
        debug_assert_eq!(0, max_input_len % MIN_ENCODE_CHUNK_SIZE);
        debug_assert_eq!(0, input_chunks_to_encode_len % MIN_ENCODE_CHUNK_SIZE);

        encoded_size += self.engine.internal_encode(
            &input[..(input_chunks_to_encode_len)],
            &mut self.output[encoded_size..],
        );

        // not updating `self.output_occupied_len` here because if the below write fails, it should
        // "never take place" -- the buffer contents we encoded are ignored and perhaps retried
        // later, if the consumer chooses.

        self.write_to_delegate(encoded_size)
            // no matter whether we wrote the full encoded buffer or not, we consumed the same
            // input
            .map(|()| extra_input_read_len + input_chunks_to_encode_len)
            .map_err(|e| {
                // in case we filled and encoded `extra`, reset extra_len
                self.extra_input_occupied_len = orig_extra_len;

                e
            })
    }