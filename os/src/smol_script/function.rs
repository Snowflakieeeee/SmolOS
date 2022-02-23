use alloc::borrow::Cow;

use super::{node::Node, types::Type};

#[derive(Clone)]
pub struct Function {
    name: Cow<'static, str>,
    args: Cow<'static, [Type]>,
    ret: Type,
    function: fn(&[Node]) -> Node,
}

impl Function {
    pub const fn new(
        name: &'static str,
        function: fn(&[Node]) -> Node,
        args: &'static [Type],
        ret: Type,
    ) -> Self {
        Self {
            name: Cow::Borrowed(name),
            args: Cow::Borrowed(args),
            ret,
            function,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn ret(&self) -> Type {
        self.ret
    }

    pub fn args(&self) -> &[Type] {
        self.args.as_ref()
    }
}
