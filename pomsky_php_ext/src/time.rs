use chrono::prelude::*;
use ext_php_rs::prelude::*;

#[php_class(name ="Chrono\\LocalDateTime")]
pub struct LocalDateTime {
    inner: DateTime<Local>
}

#[php_impl]
impl LocalDateTime {
    pub fn now() -> Self {
        LocalDateTime { inner: Local::now() }
    }

    pub fn format(&self, format: &str) -> String {
        self.inner.format(format).to_string()
    }
}
