use pomsky::options::{CompileOptions, RegexFlavor};
use pomsky::Expr;

use ext_php_rs::prelude::*;

use crate::time::LocalDateTime;

mod time;
mod router;

#[php_function(name = "pomsky\\create")]
pub fn pomsky(pattern: &str) -> Result<String, PhpException> {
    let options = CompileOptions {
        flavor: RegexFlavor::Pcre,
        ..Default::default()
    };

    match Expr::parse_and_compile(pattern, options) {
        (Some(regex), _) => Ok(regex),
        (None, diagnostics) => {
            use std::fmt::Write;

            let mut message = String::new();

            for error in diagnostics {
                write!(message, "{:?}", error).unwrap();
            }
            
            Err(PhpException::default(message))
        }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

