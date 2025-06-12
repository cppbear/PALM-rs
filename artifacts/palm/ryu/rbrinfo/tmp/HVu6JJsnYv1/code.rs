pub unsafe fn format32(f: f32, result: *mut u8) -> usize {
    let bits = f.to_bits();
    let sign = ((bits >> (FLOAT_MANTISSA_BITS + FLOAT_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u32 << FLOAT_MANTISSA_BITS) - 1);
    let ieee_exponent = (bits >> FLOAT_MANTISSA_BITS) & ((1u32 << FLOAT_EXPONENT_BITS) - 1);

    let mut index = 0isize;
    if sign {
        *result = b'-';
        index += 1;
    }

    if ieee_exponent == 0 && ieee_mantissa == 0 {
        ptr::copy_nonoverlapping(b"0.0".as_ptr(), result.offset(index), 3);
        return sign as usize + 3;
    }

    let v = f2d(ieee_mantissa, ieee_exponent);

    let length = common::decimal_length9(v.mantissa) as isize;
    let k = v.exponent as isize;
    let kk = length + k; // 10^(kk-1) <= v < 10^kk
    debug_assert!(k >= -45);

    if 0 <= k && kk <= 13 {
        // 1234e7 -> 12340000000.0
        write_mantissa(v.mantissa, result.offset(index + length));
        for i in length..kk {
            *result.offset(index + i) = b'0';
        }
        *result.offset(index + kk) = b'.';
        *result.offset(index + kk + 1) = b'0';
        index as usize + kk as usize + 2
    } else if 0 < kk && kk <= 13 {
        // 1234e-2 -> 12.34
        write_mantissa(v.mantissa, result.offset(index + length + 1));
        ptr::copy(result.offset(index + 1), result.offset(index), kk as usize);
        *result.offset(index + kk) = b'.';
        index as usize + length as usize + 1
    } else if -6 < kk && kk <= 0 {
        // 1234e-6 -> 0.001234
        *result.offset(index) = b'0';
        *result.offset(index + 1) = b'.';
        let offset = 2 - kk;
        for i in 2..offset {
            *result.offset(index + i) = b'0';
        }
        write_mantissa(v.mantissa, result.offset(index + length + offset));
        index as usize + length as usize + offset as usize
    } else if length == 1 {
        // 1e30
        *result.offset(index) = b'0' + v.mantissa as u8;
        *result.offset(index + 1) = b'e';
        index as usize + 2 + write_exponent2(kk - 1, result.offset(index + 2))
    } else {
        // 1234e30 -> 1.234e33
        write_mantissa(v.mantissa, result.offset(index + length + 1));
        *result.offset(index) = *result.offset(index + 1);
        *result.offset(index + 1) = b'.';
        *result.offset(index + length + 1) = b'e';
        index as usize
            + length as usize
            + 2
            + write_exponent2(kk - 1, result.offset(index + length + 2))
    }
}