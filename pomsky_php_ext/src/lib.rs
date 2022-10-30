use pomsky::Expr;
use pomsky::options::{CompileOptions, RegexFlavor};
//use std::ffi::{c_char, CStr, CString};

use ext_php_rs::prelude::*;

#[php_function]
pub fn pomsky(pattern: &str) -> String {
    let options = CompileOptions { flavor: RegexFlavor::Pcre, ..Default::default() };
    Expr::parse_and_compile(pattern, options).map_or(String::new(), |(r,_)| r)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

//#[no_mangle]
//pub extern fn pomsky_create(s: *const c_char) -> *mut c_char {
//    let c_str = unsafe { CStr::from_ptr(s) };
//  
//   let regex = CString::new(regex).unwrap();
//    regex.into_raw()
//}
