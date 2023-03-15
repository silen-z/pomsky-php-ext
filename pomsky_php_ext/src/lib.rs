use pomsky::options::{CompileOptions, RegexFlavor};
use pomsky::Expr;

use ext_php_rs::prelude::*;

#[php_function]
pub fn pomsky(pattern: &str) -> Result<String, PhpException> {
    let options = CompileOptions {
        flavor: RegexFlavor::Pcre,
        ..Default::default()
    };
    match Expr::parse_and_compile(pattern, options) {
        Ok((regex, _)) => Ok(regex),
        Err(e) => Err(PhpException::default(e.to_string())),
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
