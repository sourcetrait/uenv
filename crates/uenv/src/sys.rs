use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Execute;
impl Variant for Execute {
    type EnumType = Sys;
    type ValType = Cow<'static, Path>;
    const VARIANT: Sys = Sys::Execute;
}

impl AsRef<str> for Execute {
    fn as_ref(&self) -> &str {
        <Self as Variant>::VARIANT.name()
    }
}

impl From<Execute> for Sys {
    fn from(value: Execute) -> Self {
        Execute::VARIANT
    }
}

impl TryFrom<Execute> for Var {
    type Error = Execute;

    fn try_from(value: Execute) -> Result<Self, Self::Error> {
        Err(value)
    }
}
