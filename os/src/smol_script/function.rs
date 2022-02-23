use alloc::borrow::Cow;

#[derive(Clone)]
pub struct Function {
    name: Cow<'static, str>,
}

impl Function {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
