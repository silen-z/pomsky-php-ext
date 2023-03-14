use pomsky::options::{CompileOptions, RegexFlavor};
use pomsky::Expr;
use std::ffi::{c_char, CStr, CString};

// use ext_php_rs::prelude::*;

// #[php_function]
// pub fn pomsky(pattern: &str) -> String {
//     let options = CompileOptions {
//         flavor: RegexFlavor::Pcre,
//         ..Default::default()
//     };
//     Expr::parse_and_compile(pattern, options).map_or(String::new(), |(r, _)| r)
// }

// #[php_module]
// pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
//     module
// }

#[no_mangle]
pub unsafe extern "C" fn pomsky_create(pattern: *const c_char) -> *mut i8 {
    let pattern = CStr::from_ptr(pattern).to_str().unwrap();

    let options = CompileOptions {
        flavor: RegexFlavor::Pcre,
        ..Default::default()
    };

    match Expr::parse_and_compile(pattern, options) {
        Ok((regex, _)) => {
            let regex = CString::new(regex).unwrap();
            regex.into_raw()
        }
        _ => std::ptr::null_mut(),
    }
}
