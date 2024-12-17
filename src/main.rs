use std::arch::global_asm;
use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn test() {
    println!("test");
}

extern "C" {
    fn bar() -> c_int;
}

global_asm!(".global foo", ".type foo,%function", ".set foo, test");
global_asm!(
    ".global foobar",
    ".type foobar,%function",
    ".set foobar, bar"
);

unsafe extern "C" {
    fn foo();
    fn foobar() -> c_int;
}

fn main() {
    unsafe { foo() };
    println!("{}", unsafe { bar() });
    println!("{:?}", unsafe { foobar() });
}
