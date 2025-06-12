#![feature(int_log)]
#![feature(allocator_api)]

#[macro_use]
extern crate afl;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn test_function174(_param0 :u16) {
    let _local0 = http::status::StatusCode::from_u16(_param0);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    let _local1_param0_helper2 = &(_local1_param0_helper1);
    let _ = http::status::StatusCode::as_str(_local1_param0_helper2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u16(data, 0);
        test_function174(_param0);
    });
}
