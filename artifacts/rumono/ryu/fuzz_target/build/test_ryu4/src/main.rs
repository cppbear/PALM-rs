#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}

fn test_function4(_param0 :f64 ,mut _param1 :u8) {
    unsafe {
        let _local0_param1_helper1 = &(_param1) as *mut u8;
        let _ = ryu::raw::format64(_param0, _local0_param1_helper1);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 9 {return;}
        let _param0 = _to_f64(data, 0);
        let _param1 = _to_u8(data, 8);
        test_function4(_param0 ,_param1);
    });
}
