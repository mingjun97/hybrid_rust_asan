extern crate cty;
use std::env;
extern "C"{
    fn array_from_rust(_: * const cty::c_int ,_: cty::c_int ) -> cty::c_int;
    fn array_in_lib(_: cty::c_int) -> cty::c_int;
    fn array_to_rust(_: cty::c_int) -> *mut cty::c_int;
} 


fn main() {
    let args: Vec<String> = env::args().collect();
    let option = if args.len() < 2 {
        "undefine"
    }else{
        &args[1]
    };
    let program = &args[0];
    let xs = [0, 1, 2, 3];
    match option {
        "1" => {
            let _y = unsafe{ array_from_rust(xs.as_ptr(), 5)}; "Array from rust";
        },
        "2" => {
            let _y = unsafe{array_in_lib(101);}; "Array in lib";
        },
        "3" => {
            let size = 5;
            let arr: *mut i32 = unsafe{array_to_rust(size)};
            let _arr_rust = unsafe{std::slice::from_raw_parts(arr as *const i32, (size + 10) as usize)};
            let _v = unsafe{*(arr.offset(6))} ; "Array from library";
        },
        _ => {
            println!("Usage {} NUMBER\n NUMBER= 1, 2, or 3", program);
            "Args required";
        }
    }

}