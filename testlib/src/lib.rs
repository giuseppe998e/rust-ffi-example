use libc::c_int;

#[no_mangle]
pub extern "C" fn rust_sum(x: c_int, y: c_int) -> c_int {
  x + y
}

#[no_mangle]
pub extern "C" fn rust_sub(x: c_int, y: c_int) -> c_int {
  x - y
}

#[no_mangle]
pub extern "C" fn rust_mul(x: c_int, y: c_int) -> c_int {
  x * y
}

#[no_mangle]
pub extern "C" fn rust_div(x: c_int, y: c_int) -> c_int {
  x / y
}
