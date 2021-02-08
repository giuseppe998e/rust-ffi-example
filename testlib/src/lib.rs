#[no_mangle]
pub extern "C" fn rust_sum(x: i32, y: i32) -> i32 {
  x + y
}

#[no_mangle]
pub extern "C" fn rust_sub(x: i32, y: i32) -> i32 {
  x - y
}

#[no_mangle]
pub extern "C" fn rust_mul(x: i32, y: i32) -> i32 {
  x * y
}

#[no_mangle]
pub extern "C" fn rust_div(x: i32, y: i32) -> i32 {
  x / y
}
