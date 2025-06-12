#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn test_function1(_param0 :i8) {
    let mut _local0 = <itoa::Buffer as std::default::Default>::default();
    let _local1_param0_helper1 = &mut (_local0);
    let _: &str = itoa::Buffer::format(_local1_param0_helper1, _param0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_i8(data, 0);
        test_function1(_param0);
    });
}
