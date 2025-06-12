use crate::{
    engine::{general_purpose::INVALID_VALUE, DecodeMetadata, DecodePaddingMode},
    DecodeError, DecodeSliceError, PAD_BYTE,
};

/// Decode the last 0-4 bytes, checking for trailing set bits and padding per the provided
/// parameters.
///
/// Returns the decode metadata representing the total number of bytes decoded, including the ones
/// indicated as already written by `output_index`.
pub(crate) fn decode_suffix(
    input: &[u8],
    input_index: usize,
    output: &mut [u8],
    mut output_index: usize,
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding_mode: DecodePaddingMode,
) -> Result<DecodeMetadata, DecodeSliceError> {
    debug_assert!((input.len() - input_index) <= 4);

    // Decode any leftovers that might not be a complete input chunk of 4 bytes.
    // Use a u32 as a stack-resident 4 byte buffer.
    let mut morsels_in_leftover = 0;
    let mut padding_bytes_count = 0;
    // offset from input_index
    let mut first_padding_offset: usize = 0;
    let mut last_symbol = 0_u8;
    let mut morsels = [0_u8; 4];

    for (leftover_index, &b) in input[input_index..].iter().enumerate() {
        // '=' padding
        if b == PAD_BYTE {
            // There can be bad padding bytes in a few ways:
            // 1 - Padding with non-padding characters after it
            // 2 - Padding after zero or one characters in the current quad (should only
            //     be after 2 or 3 chars)
            // 3 - More than two characters of padding. If 3 or 4 padding chars
            //     are in the same quad, that implies it will be caught by #2.
            //     If it spreads from one quad to another, it will be an invalid byte
            //     in the first quad.
            // 4 - Non-canonical padding -- 1 byte when it should be 2, etc.
            //     Per config, non-canonical but still functional non- or partially-padded base64
            //     may be treated as an error condition.

            if leftover_index < 2 {
                // Check for error #2.
                // Either the previous byte was padding, in which case we would have already hit
                // this case, or it wasn't, in which case this is the first such error.
                debug_assert!(
                    leftover_index == 0 || (leftover_index == 1 && padding_bytes_count == 0)
                );
                let bad_padding_index = input_index + leftover_index;
                return Err(DecodeError::InvalidByte(bad_padding_index, b).into());
            }

            if padding_bytes_count == 0 {
                first_padding_offset = leftover_index;
            }

            padding_bytes_count += 1;
            continue;
        }

        // Check for case #1.
        // To make '=' handling consistent with the main loop, don't allow
        // non-suffix '=' in trailing chunk either. Report error as first
        // erroneous padding.
        if padding_bytes_count > 0 {
            return Err(
                DecodeError::InvalidByte(input_index + first_padding_offset, PAD_BYTE).into(),
            );
        }

        last_symbol = b;

        // can use up to 8 * 6 = 48 bits of the u64, if last chunk has no padding.
        // Pack the leftovers from left to right.
        let morsel = decode_table[b as usize];
        if morsel == INVALID_VALUE {
            return Err(DecodeError::InvalidByte(input_index + leftover_index, b).into());
        }

        morsels[morsels_in_leftover] = morsel;
        morsels_in_leftover += 1;
    }

    // If there was 1 trailing byte, and it was valid, and we got to this point without hitting
    // an invalid byte, now we can report invalid length
    if !input.is_empty() && morsels_in_leftover < 2 {
        return Err(DecodeError::InvalidLength(input_index + morsels_in_leftover).into());
    }

    match padding_mode {
        DecodePaddingMode::Indifferent => { /* everything we care about was already checked */ }
        DecodePaddingMode::RequireCanonical => {
            // allow empty input
            if (padding_bytes_count + morsels_in_leftover) % 4 != 0 {
                return Err(DecodeError::InvalidPadding.into());
            }
        }
        DecodePaddingMode::RequireNone => {
            if padding_bytes_count > 0 {
                // check at the end to make sure we let the cases of padding that should be InvalidByte
                // get hit
                return Err(DecodeError::InvalidPadding.into());
            }
        }
    }

    // When encoding 1 trailing byte (e.g. 0xFF), 2 base64 bytes ("/w") are needed.
    // / is the symbol for 63 (0x3F, bottom 6 bits all set) and w is 48 (0x30, top 2 bits
    // of bottom 6 bits set).
    // When decoding two symbols back to one trailing byte, any final symbol higher than
    // w would still decode to the original byte because we only care about the top two
    // bits in the bottom 6, but would be a non-canonical encoding. So, we calculate a
    // mask based on how many bits are used for just the canonical encoding, and optionally
    // error if any other bits are set. In the example of one encoded byte -> 2 symbols,
    // 2 symbols can technically encode 12 bits, but the last 4 are non-canonical, and
    // useless since there are no more symbols to provide the necessary 4 additional bits
    // to finish the second original byte.

    let leftover_bytes_to_append = morsels_in_leftover * 6 / 8;
    // Put the up to 6 complete bytes as the high bytes.
    // Gain a couple percent speedup from nudging these ORs to use more ILP with a two-way split.
    let mut leftover_num = (u32::from(morsels[0]) << 26)
        | (u32::from(morsels[1]) << 20)
        | (u32::from(morsels[2]) << 14)
        | (u32::from(morsels[3]) << 8);

    // if there are bits set outside the bits we care about, last symbol encodes trailing bits that
    // will not be included in the output
    let mask = !0_u32 >> (leftover_bytes_to_append * 8);
    if !decode_allow_trailing_bits && (leftover_num & mask) != 0 {
        // last morsel is at `morsels_in_leftover` - 1
        return Err(DecodeError::InvalidLastSymbol(
            input_index + morsels_in_leftover - 1,
            last_symbol,
        )
        .into());
    }

    // Strangely, this approach benchmarks better than writing bytes one at a time,
    // or copy_from_slice into output.
    for _ in 0..leftover_bytes_to_append {
        let hi_byte = (leftover_num >> 24) as u8;
        leftover_num <<= 8;
        *output
            .get_mut(output_index)
            .ok_or(DecodeSliceError::OutputSliceTooSmall)? = hi_byte;
        output_index += 1;
    }

    Ok(DecodeMetadata::new(
        output_index,
        if padding_bytes_count > 0 {
            Some(input_index + first_padding_offset)
        } else {
            None
        },
    ))
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use engine::Config;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::error::Error;
	use std::convert::From;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut usize_0: usize = 231usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_0: u8 = 39u8;
    let mut usize_1: usize = 3594usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_1, u8_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut u8_1: u8 = 33u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut u8_2: u8 = 100u8;
    let mut u8_3: u8 = 114u8;
    let mut bool_0: bool = false;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_0, bool_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_3);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut parsealphabeterror_2: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_2);
    let mut option_0: std::option::Option<&dyn std::error::Error> = crate::decode::DecodeSliceError::source(decodesliceerror_2_ref_0);
    let mut parsealphabeterror_2_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_2;
    let mut bool_1: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_2_ref_0, parsealphabeterror_0_ref_0);
    let mut parsealphabeterror_3: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_1);
    let mut tuple_0: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_1_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut bool_0: bool = false;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut usize_0: usize = 2748usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_0);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_0: u8 = 22u8;
    let mut usize_1: usize = 2997usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_1, u8_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut u8_1: u8 = 22u8;
    let mut usize_2: usize = 3375usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_1);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut u8_2: u8 = 108u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_2);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_2, decodepaddingmode_1);
    let mut tuple_0: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_2);
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    let mut generalpurposeconfig_3_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_3;
    let mut bool_1: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_3_ref_0);
    let mut generalpurposeconfig_4: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_1_ref_0);
    panic!("From RustyUnit with love");
}
}