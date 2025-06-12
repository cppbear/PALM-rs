fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        // offset == BUF_SIZE when we copied it all last time
        debug_assert!(self.b64_offset <= BUF_SIZE);
        debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
        debug_assert!(if self.b64_offset == BUF_SIZE {
            self.b64_len == 0
        } else {
            self.b64_len <= BUF_SIZE
        });

        debug_assert!(if self.decoded_len == 0 {
            // can be = when we were able to copy the complete chunk
            self.decoded_offset <= DECODED_CHUNK_SIZE
        } else {
            self.decoded_offset < DECODED_CHUNK_SIZE
        });

        // We shouldn't ever decode into decoded_buffer when we can't immediately write at least one
        // byte into the provided buf, so the effective length should only be 3 momentarily between
        // when we decode and when we copy into the target buffer.
        debug_assert!(self.decoded_len < DECODED_CHUNK_SIZE);
        debug_assert!(self.decoded_len + self.decoded_offset <= DECODED_CHUNK_SIZE);

        if self.decoded_len > 0 {
            // we have a few leftover decoded bytes; flush that rather than pull in more b64
            self.flush_decoded_buf(buf)
        } else {
            let mut at_eof = false;
            while self.b64_len < BASE64_CHUNK_SIZE {
                // Copy any bytes we have to the start of the buffer.
                self.b64_buffer
                    .copy_within(self.b64_offset..self.b64_offset + self.b64_len, 0);
                self.b64_offset = 0;

                // then fill in more data
                let read = self.read_from_delegate()?;
                if read == 0 {
                    // we never read into an empty buf, so 0 => we've hit EOF
                    at_eof = true;
                    break;
                }
            }

            if self.b64_len == 0 {
                debug_assert!(at_eof);
                // we must be at EOF, and we have no data left to decode
                return Ok(0);
            };

            debug_assert!(if at_eof {
                // if we are at eof, we may not have a complete chunk
                self.b64_len > 0
            } else {
                // otherwise, we must have at least one chunk
                self.b64_len >= BASE64_CHUNK_SIZE
            });

            debug_assert_eq!(0, self.decoded_len);

            if buf.len() < DECODED_CHUNK_SIZE {
                // caller requested an annoyingly short read
                // have to write to a tmp buf first to avoid double mutable borrow
                let mut decoded_chunk = [0_u8; DECODED_CHUNK_SIZE];
                // if we are at eof, could have less than BASE64_CHUNK_SIZE, in which case we have
                // to assume that these last few tokens are, in fact, valid (i.e. must be 2-4 b64
                // tokens, not 1, since 1 token can't decode to 1 byte).
                let to_decode = cmp::min(self.b64_len, BASE64_CHUNK_SIZE);

                let decoded = self.decode_to_buf(to_decode, &mut decoded_chunk[..])?;
                self.decoded_chunk_buffer[..decoded].copy_from_slice(&decoded_chunk[..decoded]);

                self.decoded_offset = 0;
                self.decoded_len = decoded;

                // can be less than 3 on last block due to padding
                debug_assert!(decoded <= 3);

                self.flush_decoded_buf(buf)
            } else {
                let b64_bytes_that_can_decode_into_buf = (buf.len() / DECODED_CHUNK_SIZE)
                    .checked_mul(BASE64_CHUNK_SIZE)
                    .expect("too many chunks");
                debug_assert!(b64_bytes_that_can_decode_into_buf >= BASE64_CHUNK_SIZE);

                let b64_bytes_available_to_decode = if at_eof {
                    self.b64_len
                } else {
                    // only use complete chunks
                    self.b64_len - self.b64_len % 4
                };

                let actual_decode_len = cmp::min(
                    b64_bytes_that_can_decode_into_buf,
                    b64_bytes_available_to_decode,
                );
                self.decode_to_buf(actual_decode_len, buf)
            }
        }
    }