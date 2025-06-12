#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_f32(data:&[u8], index: usize) -> f32 {
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}

fn test_function5(_param0 :f32 ,mut _param1 :u8) {
    unsafe {
        let _local0_param1_helper1 = &(_param1) as *mut u8;
        let _ = ryu::raw::format32(_param0, _local0_param1_helper1);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 5 {return;}
        let _param0 = _to_f32(data, 0);
        let _param1 = _to_u8(data, 4);
        test_function5(_param0 ,_param1);
    });
}
