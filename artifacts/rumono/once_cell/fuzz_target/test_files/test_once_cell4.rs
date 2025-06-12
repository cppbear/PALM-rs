#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn test_function4(mut _param0 :u8) {
    let mut _local0_param0_helper1 = Some(_param0);
    let _local0_param0_helper2 = &(_local0_param0_helper1) as *mut std::option::Option::<u8>;
    let _local0: once_cell::unsync::OnceCell::<*std::option::Option::<u8>> = once_cell::unsync::OnceCell::<*std::option::Option::<u8>>::with_value(_local0_param0_helper2);
    let _: std::option::Option::<*std::option::Option::<u8>> = once_cell::unsync::OnceCell::<*std::option::Option::<u8>>::into_inner(_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function4(_param0);
    });
}
